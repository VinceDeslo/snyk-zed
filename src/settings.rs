// pub struct SnykLSPSettings {
//     activateSnykOpenSource = ps.ossScanEnable.toString(),
//     activateSnykCodeSecurity = ps.snykCodeSecurityIssuesScanEnable.toString(),
//     activateSnykCodeQuality = ps.snykCodeQualityIssuesScanEnable.toString(),
//     activateSnykIac = isSnykIaCLSEnabled().toString(),
//     organization = ps.organization,
//     insecure = ps.ignoreUnknownCA.toString(),
//     endpoint = getEndpointUrl(),
//     cliPath = getCliFile().absolutePath,
//     token = ps.token,
//     filterSeverity =
//     SeverityFilter(
//         critical = ps.criticalSeverityEnabled,
//         high = ps.highSeverityEnabled,
//         medium = ps.mediumSeverityEnabled,
//         low = ps.lowSeverityEnabled,
//     ),
//     enableTrustedFoldersFeature = "false",
//     scanningMode = if (!ps.scanOnSave) "manual" else "auto",
//     integrationName = pluginInfo.integrationName,
//     integrationVersion = pluginInfo.integrationVersion,
//     authenticationMethod = authMethod,
//     enableSnykOSSQuickFixCodeActions = "true",
// }
