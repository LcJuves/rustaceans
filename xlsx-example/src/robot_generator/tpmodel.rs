use super::one_case::OneCase;
use super::tpauth::AUTH_CONF;
use super::tputil::UA;
use crate::seeval;
use crate::time_millis_string;
use crate::util::hyper::*;

use std::collections::HashMap;
use std::error::Error;

use hyper::{Body, Response};
use regex::Regex;
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

lazy_static! {
    static ref DOC_REP_REGEX1: Regex = Regex::new(r"<br[^>]*>").unwrap();
    static ref DOC_REP_REGEX2: Regex = Regex::new(r"<[^>]*>").unwrap();
}

async fn _this_common_req_get(
    url: &str,
    sessionid: &str,
    ep_jwt_token_current: &str,
    project_id: Option<&u8>,
) -> Result<Response<Body>, Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert(
        "Cookie".to_owned(),
        format!("sessionid={};ep_jwt_token_current={}", sessionid, ep_jwt_token_current),
    );
    headers.insert("Connection".to_owned(), "keep-alive".to_owned());
    if let Some(pid) = project_id {
        headers.insert("project-id".to_owned(), pid.to_string());
    }

    Ok(get(url, &headers).await?)
}

pub(crate) async fn query_project_id_by_name(
    sessionid: &str,
    ep_jwt_token_current: &str,
    project_name: &str,
) -> Result<u8, Box<dyn Error>> {
    let url = format!("http://199.200.0.8/api/v1/projects/set/?_t={}", time_millis_string!());
    let resp = _this_common_req_get(&url, sessionid, ep_jwt_token_current, None).await?;
    let resp_json_string = resp_json_string_from(resp).await?;
    seeval!(&resp_json_string);
    #[derive(Serialize, Deserialize, Debug)]
    struct ItemData {
        id: u8,
        name: String,
    }
    let resp_data: Vec<ItemData> = serde_json::from_str(&resp_json_string)?;
    seeval!(&resp_data);
    for item_data in resp_data {
        if item_data.name == project_name {
            return Ok(item_data.id);
        }
    }
    Ok(0u8)
}

pub(crate) async fn query_remote_root_path_id_by_name(
    sessionid: &str,
    ep_jwt_token_current: &str,
    project_id: &u8,
    remote_root_path_name: &str,
) -> Result<u8, Box<dyn Error>> {
    let url = format!(
        "http://199.200.0.8/api/v1/versions/?project_id={}&index=true&_t={}",
        project_id,
        time_millis_string!()
    );
    let resp =
        _this_common_req_get(&url, sessionid, ep_jwt_token_current, Some(project_id)).await?;
    let resp_json_string = resp_json_string_from(resp).await?;
    seeval!(&resp_json_string);
    #[derive(Serialize, Deserialize, Debug)]
    struct ItemData {
        id: u8,
        name: String,
        text: String,
        value: u8,
    }
    let resp_data: Vec<ItemData> = serde_json::from_str(&resp_json_string)?;
    seeval!(&resp_data);
    for item_data in resp_data {
        if item_data.name == remote_root_path_name {
            return Ok(item_data.id);
        }
    }
    Ok(0u8)
}

fn replace_special_content(doc: &str) -> String {
    let ret = doc.to_owned();
    let ref doc_rep_regex1 = DOC_REP_REGEX1;
    let ref doc_rep_regex2 = DOC_REP_REGEX2;

    let ret = doc_rep_regex1.replace_all(&ret, "\n");
    let ret = doc_rep_regex2.replace_all(&ret, "");
    let ret = ret.to_string();
    let ret = ret.replace("&nbsp;", " ");
    let ret = ret.replace("&amp;", "&");
    let ret = ret.replace("&lt;", "<");
    let ret = ret.replace("&gt;", ">");
    let ret = ret.replace("&quot;", "\"");
    let ret = ret.replace("&hellip;", "…");
    let ret = ret.replace("&mdash;", "—");
    let ret = ret.replace("&ndash;", "–");
    let ret = ret.replace("&lsquo;", "‘");
    let ret = ret.replace("&rsquo;", "’");
    let ret = ret.replace("&ldquo;", "“");
    let ret = ret.replace("&rdquo;", "”");
    let ret = ret.replace("&copy;", "©");
    let ret = ret.replace("&reg;", "®");
    let ret = ret.replace("&euro;", "€");
    let ret = ret.replace("&pi;", "π");
    let ret = ret.replace("&larr;", "←");
    let ret = ret.replace("&uarr;", "↑");
    let ret = ret.replace("&rarr;", "→");
    let ret = ret.replace("&darr;", "↓");
    let ret = ret.replace("&ensp;", " ");
    let ret = ret.replace("&emsp;", " ");
    let ret = ret.replace("&real;", "ℜ");
    let ret = ret.replace("&congdot;", "⩭");
    let ret = ret.replace("&cong;", "≅");
    let ret = ret.replace("&empty;", "∅");
    let ret = ret.replace("&lambda;", "λ");
    let ret = ret.replace("&lang;", "⟨");
    let ret = ret.replace("&image;", "ℑ");
    let ret = ret.replace("&alpha;", "α");
    let ret = ret.replace("&beta;", "β");
    let ret = ret.replace("&and;", "∧");
    let ret = ret.replace("&or;", "∨");
    let ret = ret.replace("&not;", "¬");
    seeval!(ret);
    ret
}

pub(crate) async fn query_cases_by_remote_path(
    sessionid: &str,
    ep_jwt_token_current: &str,
    project_id: &u8,
    remote_root_path_id: &u8,
    remote_path: &str,
) -> Result<Vec<OneCase>, Box<dyn Error>> {
    let mut remote_path_spilt_arr = remote_path.split("/").collect::<Vec<&str>>();
    remote_path_spilt_arr.push("");

    let mut node_id = "-1";

    let mut case_feature_name = "";
    let mut cases = Vec::<OneCase>::new();

    for path in remote_path_spilt_arr {
        let url = format!(
            "http://199.200.0.8/api/v1/versions/{}/case_tree/?node_id={}&version_id={}&{{}}&_={}",
            remote_root_path_id,
            node_id,
            remote_root_path_id,
            time_millis_string!()
        );

        let resp =
            _this_common_req_get(&url, sessionid, ep_jwt_token_current, Some(project_id)).await?;
        let resp_json = resp_json_from(resp).await?;
        seeval!(&resp_json);

        if let serde_json::Value::Array(arr) = &resp_json {
            for item in arr {
                if let serde_json::Value::String(icon_skin) = &item["iconSkin"] {
                    if icon_skin == "folder" {
                        let name = (&item["name"]).as_str().unwrap();
                        if name == path {
                            node_id = Box::leak(
                                (&item["dir_code"]).as_str().unwrap().to_string().into_boxed_str(),
                            );
                            case_feature_name = Box::leak(
                                (&item["path"]).as_str().unwrap_or("").to_string().into_boxed_str(),
                            );
                            continue;
                        }
                    } else if !icon_skin.is_empty()
                        && !(&item["case_code"]).as_str().unwrap().is_empty()
                    {
                        let remote_case_id = (&item["id"]).as_u64().unwrap();
                        let case_req_url = format!(
                            "http://199.200.0.8/api/v1/versions/{}/cases/{}/?project_id={}&_t={}",
                            remote_root_path_id,
                            remote_case_id,
                            project_id,
                            time_millis_string!()
                        );
                        let case_req_resp = _this_common_req_get(
                            &case_req_url,
                            sessionid,
                            ep_jwt_token_current,
                            Some(project_id),
                        )
                        .await?;
                        let case_req_resp_json = resp_json_from(case_req_resp).await?;
                        seeval!(&case_req_resp_json);

                        let mut case = OneCase::default();
                        case.feature_name = case_feature_name.to_owned();
                        case.case_id =
                            (&case_req_resp_json["case_code"]).as_str().unwrap_or("").to_owned();
                        case.case_title =
                            (&case_req_resp_json["name"]).as_str().unwrap_or("").to_owned();
                        case.preconditions = replace_special_content(
                            (&case_req_resp_json["doc_pre"]).as_str().unwrap_or(""),
                        );
                        case.steps = replace_special_content(
                            (&case_req_resp_json["doc_step"]).as_str().unwrap_or(""),
                        );
                        case.postcondition = replace_special_content(
                            (&case_req_resp_json["doc_post"]).as_str().unwrap_or(""),
                        );
                        case.desired_result = replace_special_content(
                            (&case_req_resp_json["doc_except"]).as_str().unwrap_or(""),
                        );
                        case.test_methods =
                            (&case_req_resp_json["test_method"]).as_str().unwrap_or("").to_owned();
                        case.use_case_type =
                            (&case_req_resp_json["case_type"]).as_str().unwrap_or("").to_owned();
                        case.can_be_automated =
                            (if (&case_req_resp_json["isautomated"]).as_u64().unwrap_or(0) == 1 {
                                "是"
                            } else {
                                "否"
                            })
                            .to_owned();
                        let mut case_tags = String::new();
                        for item in (&case_req_resp_json["tags"]).as_array().unwrap() {
                            case_tags.push_str(item.as_str().unwrap_or(""));
                            case_tags.push_str("    ");
                        }
                        case.tag = case_tags[..(case_tags.rfind(" ").unwrap_or(case_tags.len()))]
                            .to_string();
                        case.author = (&case_req_resp_json["author_username"])
                            .as_str()
                            .unwrap_or("")
                            .to_owned();
                        case.use_case_level =
                            (&case_req_resp_json["priority"]).as_str().unwrap_or("").to_owned();
                        case.online_question_id =
                            (&case_req_resp_json["bug_id"]).as_str().unwrap_or("").to_owned();
                        case.notes = replace_special_content(
                            (&case_req_resp_json["doc"]).as_str().unwrap_or(""),
                        );

                        cases.push(case);
                    } else {
                        panic!();
                    }
                } else if let serde_json::Value::String(json) = &resp_json {
                    eprintln!("{} >>> {}:{}", file!(), line!(), column!());
                    eprintln!("{}", json);
                    std::process::exit(-1);
                }
            }
        }
    }
    Ok(cases)
}

pub(crate) async fn query_cases(
    project_name: &str,
    remote_root_path_name: &str,
    remote_path: &str,
) -> Result<Vec<OneCase>, Box<dyn Error>> {
    let auth_conf = AUTH_CONF.as_ref()?;
    seeval!(&auth_conf);

    let sessionid = &auth_conf.sessionid();
    let ep_jwt_token_current = &auth_conf.ep_jwt_token_current();

    let project_id =
        query_project_id_by_name(sessionid, ep_jwt_token_current, project_name).await?;
    seeval!(&project_id);

    let remote_root_path_id = query_remote_root_path_id_by_name(
        sessionid,
        ep_jwt_token_current,
        &project_id,
        remote_root_path_name,
    )
    .await?;
    seeval!(&remote_root_path_id);

    let cases = query_cases_by_remote_path(
        sessionid,
        ep_jwt_token_current,
        &project_id,
        &remote_root_path_id,
        remote_path,
    )
    .await?;

    Ok(cases)
}
