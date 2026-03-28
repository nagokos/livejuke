import * as z from "zod";

export const registerSchema = z.object({
	display_name: z
		.string()
		.min(1, "表示名は必須項目です")
		.max(30, "３０文字以内にしてください"),
	email: z.email({
		error: (issue) =>
			issue.input === ""
				? "メールアドレスは必須です"
				: "メールアドレスの形式が正しくありません",
	}),
	password: z
		.string()
		.min(8, "８文字以上にしてください")
		.max(128, "１２８文字以下にしてください"),
});

export const loginSchema = z.object({
	email: z.email({
		error: (issue) =>
			issue.input === ""
				? "メールアドレスは必須です"
				: "メールアドレスの形式が正しくありません",
	}),
	password: z.string().min(1, "パスワードを入力してください"),
});

export type LoginFormValues = z.infer<typeof loginSchema>;
export type RegisterFormValues = z.infer<typeof registerSchema>;
