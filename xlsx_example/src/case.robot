*** Settings ***
Resource    resource${/}common${/}common.robot

*** Test Cases ***
{{caseTitle}}
    [Tags]        ID-{{caseId}}    {{useCaseLevel}}    UnNamedAuthor    UnNamedModule    UnImpl
    [Teardown]    CaseTeardown

    {{preconditions}}

    登录控制台    ${SdpConsoleUrl}    ${SdpConsoleUser}    ${SdpConsolePasswd}    expect=true

    {{steps}}

    {{desiredResult}}

    {{notes}}

*** Keywords ***
CaseTeardown

    {{postcondition}}