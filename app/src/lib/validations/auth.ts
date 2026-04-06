import * as v from "valibot";

export const sendCodeSchema = v.object({
	email: v.pipe(
		v.string(),
		v.nonEmpty("メールアドレスは必須です"),
		v.email("メールアドレスの形式が正しくありません"),
	),
});

export const verifyCodeSchema = v.object({
	email: v.pipe(
		v.string(),
		v.nonEmpty("メールアドレスは必須です"),
		v.email("メールアドレスの形式が正しくありません"),
	),
	code: v.pipe(
		v.string(),
		v.nonEmpty("認証コードは必須です"),
		v.length(6, "認証コードは6文字です"),
	),
});

export type SendCodeFormValues = v.InferOutput<typeof sendCodeSchema>;
export type VerifyCodeFormValues = v.InferOutput<typeof verifyCodeSchema>;
