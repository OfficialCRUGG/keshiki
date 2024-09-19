export type Button = {
  shortLabel: string;
  longLabel: string;
  key?: string;
  action: () => void;
  activeFn: () => boolean;
  active?: boolean;
};
