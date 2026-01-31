import { notification } from "$lib/stores";

/**
 * Shows a notification with the given message.
 *
 * @param {string} message - The message to display in the notification.
 */
export const showNotification = (message: string) => {
  notification.set({ message, visible: true });
};
