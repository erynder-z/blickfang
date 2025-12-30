export type AsciiCharSet =
  | "blocks"
  | "stripes"
  | "minimal"
  | "classic"
  | "extended"
  | "dense_gradient"
  | "line_art"
  | "high_detail";

export interface AsciiCharSetOption {
  id: AsciiCharSet;
  label: string;
}
