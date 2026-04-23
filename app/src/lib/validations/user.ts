import * as v from "valibot";

export const updateUserSchema = v.object({
	display_name: v.pipe(
		v.string(),
		v.nonEmpty("表示名は必須です"),
		v.maxLength(20, "表示名は20文字以内にしてください"),
	),
});

export type UpdateUserFormValues = v.InferOutput<typeof updateUserSchema>;
