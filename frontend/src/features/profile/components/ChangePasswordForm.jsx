import { useState } from "react";

import Input from "../../../shared/ui/Input";

const EMPTY_FORM = {
  current_password: "",
  new_password: "",
  confirm_password: "",
};

export default function ChangePasswordForm({
  isUpdating,
  error,
  success,
  onSubmit,
  onClearMessages,
}) {
  const [form, setForm] = useState(EMPTY_FORM);
  const [validationError, setValidationError] = useState(null);

  const handleChange = (event) => {
    const { name, value } = event.target;
    setForm((current) => ({ ...current, [name]: value }));
    setValidationError(null);
    onClearMessages();
  };

  const handleSubmit = async (event) => {
    event.preventDefault();

    if (form.new_password.length < 8) {
      setValidationError("New password must contain at least 8 characters.");
      return;
    }

    if (form.new_password !== form.confirm_password) {
      setValidationError("New password and confirmation do not match.");
      return;
    }

    const updated = await onSubmit(form);
    if (updated) setForm(EMPTY_FORM);
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="rounded-2xl border border-zinc-800 bg-zinc-900 p-6"
    >
      <h2 className="text-xl font-semibold">Change Password</h2>
      <p className="mt-1 text-sm text-zinc-400">
        Use your current password to protect this account change.
      </p>

      <div className="mt-6 grid gap-5 md:grid-cols-3">
        <Input
          label="Current password"
          name="current_password"
          type="password"
          autoComplete="current-password"
          value={form.current_password}
          required
          disabled={isUpdating}
          onChange={handleChange}
        />

        <Input
          label="New password"
          name="new_password"
          type="password"
          autoComplete="new-password"
          minLength={8}
          value={form.new_password}
          required
          disabled={isUpdating}
          onChange={handleChange}
        />

        <Input
          label="Confirm new password"
          name="confirm_password"
          type="password"
          autoComplete="new-password"
          minLength={8}
          value={form.confirm_password}
          required
          disabled={isUpdating}
          onChange={handleChange}
        />
      </div>

      {(validationError || error) && (
        <div className="mt-5 rounded-lg border border-red-900 bg-red-950/30 p-3 text-sm text-red-300">
          {validationError || error}
        </div>
      )}

      {success && (
        <div className="mt-5 rounded-lg border border-emerald-900 bg-emerald-950/30 p-3 text-sm text-emerald-300">
          {success}
        </div>
      )}

      <button
        type="submit"
        disabled={
          isUpdating ||
          !form.current_password ||
          !form.new_password ||
          !form.confirm_password
        }
        className="mt-6 rounded-lg bg-red-600 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-red-500 disabled:cursor-not-allowed disabled:opacity-50"
      >
        {isUpdating ? "Updating..." : "Update Password"}
      </button>
    </form>
  );
}
