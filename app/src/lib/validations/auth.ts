import * as z from "zod";

const registerSchema = z.object({
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
