export const isPageParameters = (pageParams: {
  [key: string]: string | string[] | undefined
}): pageParams is { id: string } => {
  return pageParams.id !== undefined
}
