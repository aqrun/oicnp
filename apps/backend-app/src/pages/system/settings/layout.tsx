"use client";

import type { ReactElement } from "react";
import SettingsLayout from "./SettingsLayout";

export default function Layout({
  children,
}: React.PropsWithChildren): ReactElement {
  return (
    <SettingsLayout>
      {children}
    </SettingsLayout>
  );
}
