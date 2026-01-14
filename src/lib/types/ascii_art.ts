export type AsciiCharSetId =
  | "blocks"
  | "stripes"
  | "minimal"
  | "classic"
  | "extended"
  | "dense_gradient"
  | "line_art"
  | "high_detail";

export type AsciiCharSet = {
  id: AsciiCharSetId;
  label: string;
  chars: string;
};
