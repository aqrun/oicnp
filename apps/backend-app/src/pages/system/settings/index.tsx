"use client";

import { appRoutePath } from "#src/utils/app";
import { useEffect } from "react";
import { useNavigate } from "react-router";

import { Container } from "./index.styled";

export default function SettingsPage() {
	const navigate = useNavigate();

	useEffect(() => {
		navigate(appRoutePath("/system/settings/message"), { replace: true });
	}, [navigate]);

	return (
		<Container className="h-full">
			settings index page
		</Container>
	);
}
