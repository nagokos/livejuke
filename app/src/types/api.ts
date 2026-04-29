import { components } from "@/types/schema";

export type ApiErrorCode = components["schemas"]["ErrorCode"];

export type FrontendErrorCode =
	| "GOOGLE_AUTH_CANCELLED"
	| "GOOGLE_AUTH_NO_TOKEN"
	| "GOOGLE_SDK_ERROR"
	| "MEDIA_FOLDER_DENIED"
	| "UPLOAD_CANCELED"
	| "UPLOAD_FAILED";

export type AppErrorCode = ApiErrorCode | FrontendErrorCode;

export interface AppErrorWithResponse {
	error: {
		code: AppErrorCode;
		message: string;
	};
	response?: Response;
}
