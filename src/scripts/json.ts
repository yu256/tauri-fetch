function formatJson(json: string) {
  return JSON.stringify(JSON.parse(json), null, 2);
}

export function tryFormatJson(json: string) {
  try {
    return formatJson(json);
  } catch (e) {
    return `{"error": "${e}"}`;
  }
}
