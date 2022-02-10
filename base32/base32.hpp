#include <iostream>
#include <vector>

typedef unsigned char byte;

class Base32 {
  private:
    static byte encode_idx(int c) { return c < 26 ? c + 'a' : c - 26 + '0'; }

    static int decode_idx(byte c) {
        if (c <= '5' && c >= '0') {
            return c - '0' + 26;
        }
        if (c <= 'z' && c >= 'a') {
            return c - 'a';
        }
        throw 100;
    }

    static std::vector<byte> code(std::vector<byte> &data, int _cnt, int _pos,
                                  bool is_decode) {
        std::vector<byte> ans;
        int val, i = 0, cnt, num, pos = -1;
        while (i < data.size()) {
            cnt = _cnt, val = 0;
            while ((!is_decode || i < data.size()) && cnt--) {
                if (pos == -1) {
                    pos = _pos;
                    num = is_decode ? decode_idx(data[i++]) : data[i++];
                }
                val |= (num >> pos-- & 1) << cnt;
            }
            ans.push_back(is_decode ? val : encode_idx(val));
        }
        if (is_decode) {
            if (cnt <= pos + 1) {
                while (cnt--) {
                    *ans.rbegin() |= (num >> pos-- & 1) << cnt;
                }
                if (~pos && num & (1 << pos) - 1) {
                    throw 100;
                }
            } else {
                if (ans.back() || num & (1 << pos) - 1) {
                    throw 100;
                }
                ans.pop_back();
            }
        } else {
            while (~pos) {
                val = 0, cnt = 5;
                while (cnt-- && ~pos) {
                    val |= (num >> pos-- & 1) << cnt;
                }
                ans.push_back(encode_idx(val));
            }
        }
        return ans;
    }

  public:
    static std::vector<byte> encode(std::vector<byte> data) {
        return code(data, 5, 7, false);
    }

    static std::vector<byte> decode(std::vector<byte> data) {
        return code(data, 8, 4, true);
    }
};
