/**
 * Created at 2021/12/26 15:34
 *
 * @author Liangcheng Juves
 */

pub(crate) fn remove_eol(r#str: &str) -> String {
    let owned_str = r#str.to_owned();
    (&owned_str
        [..(owned_str.rfind("\r").unwrap_or(owned_str.rfind("\n").unwrap_or(owned_str.len())))])
        .to_string()
}
