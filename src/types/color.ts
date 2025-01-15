export enum Color {
  Primary,
  PrimaryContent,
  Secondary,
  SecondaryContent,
  Accent,
  AccentContent,
  Neutral,
  NeutralContent,
  Base100,
  Base200,
  Base300,
  BaseContent,
  Info,
  InfoContent,
  Success,
  SuccessContent,
  Warning,
  WarningContent,
  Error,
  ErrorContent,
}

export class ColorUtils {
  public static toClassName(color: Color): string {
    switch (color) {
      case Color.Primary:
        return 'primary'
      case Color.PrimaryContent:
        return 'primary-content'
      case Color.Secondary:
        return 'secondary'
      case Color.SecondaryContent:
        return 'secondary-content'
      case Color.Accent:
        return 'accent'
      case Color.AccentContent:
        return 'accent-content'
      case Color.Neutral:
        return 'neutral'
      case Color.NeutralContent:
        return 'neutral-content'
      case Color.Base100:
        return 'base-100'
      case Color.Base200:
        return 'base-200'
      case Color.Base300:
        return 'base-300'
      case Color.BaseContent:
        return 'base-content'
      case Color.Info:
        return 'info'
      case Color.InfoContent:
        return 'info-content'
      case Color.Success:
        return 'success'
      case Color.SuccessContent:
        return 'success-content'
      case Color.Warning:
        return 'warning'
      case Color.WarningContent:
        return 'warning-content'
      case Color.Error:
        return 'error'
      case Color.ErrorContent:
        return 'error-content'
      default:
        return 'primary'
    }
  }
}
