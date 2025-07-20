export interface CommandResult<T> {
  success: boolean;
  data?: T;
  message?: string;
}
