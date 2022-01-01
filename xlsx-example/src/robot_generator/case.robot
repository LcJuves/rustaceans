*** Settings ***
Resource    resource${/}common${/}common.robot

*** Test Cases ***
{{case_title}}
    [Tags]        ID-{{case_id}}    {{use_case_level}}    {{author_tag}}    {{mod_tag}}    UnImpl
    [Teardown]    CaseTeardown

    {{preconditions}}

    登录控制台    ${SdpConsoleUrl}    ${SdpConsoleUser}    ${SdpConsolePasswd}    expect=true

    {{steps}}

    {{desired_result}}

    {{notes}}

*** Keywords ***
CaseTeardown

    {{postcondition}}