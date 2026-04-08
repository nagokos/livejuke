import * as v from "valibot";

export const sendCodeSchema = v.object({
	email: v.pipe(
		v.string(),
		v.nonEmpty("メールアドレスは必須です"),
		v.email("メールアドレスの形式が正しくありません"),
	),
});

export type SendCodeFormValues = v.InferOutput<typeof sendCodeSchema>;
