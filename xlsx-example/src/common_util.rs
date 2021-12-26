/**
 * Created at 2021/12/26 15:34
 *
 * @author Liangcheng Juves
 */

pub(crate) fn remove_eol(r#str: &str) -> String {
    let owned_string = r#str.to_owned();
    (&owned_string[..(owned_string
        .rfind("\r")
        .unwrap_or(owned_string.rfind("\n").unwrap_or(owned_string.len())))])
        .to_string()
}
