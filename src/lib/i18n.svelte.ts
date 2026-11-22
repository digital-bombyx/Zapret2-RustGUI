export type Lang = "ru" | "en";

export const i18n = $state({ lang: "ru" as Lang });

/** Двуязычная строка: t("по-русски", "in English"). Реактивна в шаблонах. */
export function t(ru: string, en: string): string {
  return i18n.lang === "en" ? en : ru;
}

export const locale = () => (i18n.lang === "en" ? "en-GB" : "ru-RU");
