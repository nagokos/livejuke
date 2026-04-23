const PUBLIC_ENDPOINTS = [
	{ method: "POST", path: "/auth/send-code" },
	{ method: "POST", path: "/auth/verify-code" },
	{ method: "POST", path: "/auth/refresh" },
	{ method: "POST", path: "/auth/google" },
	{ method: "POST", path: "/auth/logout" },
];

export const isPublicEndpoint = (request: Request) => {
	return PUBLIC_ENDPOINTS.includes({
		method: request.method,
		path: new URL(request.url).pathname,
	});
};
