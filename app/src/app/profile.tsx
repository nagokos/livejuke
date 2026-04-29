import { View, Text, Pressable, TextInput } from "react-native";
import { useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Image } from "expo-image";
import { Controller, useForm } from "react-hook-form";
import { UpdateUserFormValues, updateUserSchema } from "@/lib/validations/user";
import { valibotResolver } from "@hookform/resolvers/valibot";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import { useUserUpdateMutation } from "@/hooks/useUserUpdateMutation";

export default function ProfileEdit() {
	const { currentUser } = useCurrentUser();
	const [rootError, setRootError] = useState("");
	const { updateUser, updateAvatar, isProcessing } = useUserUpdateMutation();

	const {
		control,
		handleSubmit,
		setError,
		formState: { errors },
	} = useForm<UpdateUserFormValues>({
		resolver: valibotResolver(updateUserSchema),
		defaultValues: { display_name: currentUser?.display_name },
	});

	const onUpdateAvatarSubmit = () => {
		updateAvatar.mutate(undefined, {
			onError: ({ error }) => {
				switch (error.code) {
					case "UPLOAD_CANCELED":
						return;
					case "INVALID_MEDIA_TYPE":
						setRootError("png,jpeg,webpのみアップロードできます");
						break;
					case "MEDIA_FOLDER_DENIED":
						setRootError("写真へのアクセスが許可されていません");
						break;
					case "UPLOAD_FAILED":
						setRootError("アップロードに失敗しました。再度お試しください");
						break;
					case "GLOBAL_RATE_LIMITED":
						setRootError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					default:
						setRootError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
				}
			},
			onSuccess: () => {
				setRootError("");
			},
		});
	};

	const onUpdateUserSubmit = (values: UpdateUserFormValues) => {
		updateUser.mutate(values, {
			onError: ({ error }) => {
				switch (error.code) {
					case "INVALID_DISPLAY_NAME":
						setError("display_name", {
							message: "表示名を正しく入力してください",
						});
						break;
					case "GLOBAL_RATE_LIMITED":
						setRootError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					default:
						setRootError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
				}
			},
			onSuccess: () => {
				setRootError("");
			},
		});
	};

	return (
		<View className="flex-1 bg-white px-7 justify-between">
			<View>
				<View className="items-center py-5 mt-6">
					{rootError !== "" && (
						<Text className="text-base text-red-500 mb-6">{rootError}</Text>
					)}
					<Pressable onPress={onUpdateAvatarSubmit}>
						<Avatar className="size-32 mb-2" alt="user avatar">
							<AvatarImage
								source={{ uri: currentUser?.avatar_url ?? undefined }}
							/>
							<AvatarFallback className="bg-transparent">
								<Image
									source={require("@/assets/images/user_default_avatar.png")}
									style={{ width: "100%", height: "100%" }}
								/>
							</AvatarFallback>
						</Avatar>
						<Text className="text-gray-400 text-sm font-medium text-center">
							変更する
						</Text>
					</Pressable>
				</View>
				<Controller
					name="display_name"
					control={control}
					render={({ field: { onChange, value } }) => (
						<View className="gap-2">
							<Text className="text-sm font-medium text-foreground">
								表示名
							</Text>
							<TextInput
								value={value}
								onChangeText={onChange}
								autoCapitalize="none"
								autoCorrect={false}
								className="h-12 rounded-lg border border-gray-200 bg-gray-50 pl-4 pr-10"
								placeholder="表示名を入力"
							/>
							{errors.display_name && (
								<Text className="text-sm text-red-500">
									{errors.display_name?.message}
								</Text>
							)}
						</View>
					)}
				/>
			</View>
			<Button
				onPress={handleSubmit(onUpdateUserSubmit)}
				size="lg"
				className="rounded-xl bg-main h-12 text-white mb-10"
				disabled={isProcessing}
			>
				<Text className="text-white font-bold">
					{isProcessing ? "更新中..." : "更新する"}
				</Text>
			</Button>
		</View>
	);
}
