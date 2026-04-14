import { View, Text, Pressable, TextInput } from "react-native";
import * as ImagePicker from "expo-image-picker";
import { useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { useAuthStore } from "@/stores/auth";
import { Image } from "expo-image";
import { client } from "@/api/client";
import { ErrorCode } from "@/api/error";
import { Controller, useForm } from "react-hook-form";
import { UpdateUserFormValues, updateUserSchema } from "@/lib/validations/user";
import { valibotResolver } from "@hookform/resolvers/valibot";

export default function ProfileEdit() {
	const current_user = useAuthStore((state) => state.currentUser);
	const [rootError, setRootError] = useState("");

	const {
		control,
		handleSubmit,
		getValues,
		setError,
		formState: { errors, isSubmitting },
	} = useForm<UpdateUserFormValues>({
		resolver: valibotResolver(updateUserSchema),
		defaultValues: { display_name: current_user?.display_name },
	});

	const pickImage = async () => {
		const permissionResult =
			await ImagePicker.requestMediaLibraryPermissionsAsync();

		if (!permissionResult.granted) {
			alert("写真へのアクセスが許可されていません");
			return;
		}

		const result = await ImagePicker.launchImageLibraryAsync({
			mediaTypes: ["images"],
			allowsEditing: true,
			aspect: [1, 1],
			quality: 0.8,
		});

		if (!result.canceled) {
			const media_type = result.assets[0]?.mimeType;
			if (!media_type) {
				return;
			}

			const presigned_uri = await get_presigned_uri(media_type);
			if (!presigned_uri) return;

			const uri = result.assets[0]?.uri;
			if (!uri) return;
			const response = await fetch(uri);
			const blob = await response.blob();

			try {
				const result = await fetch(presigned_uri, {
					method: "PUT",
					headers: { "Content-Type": media_type },
					body: blob,
				});

				const etag = result.headers.get("etag");
				if (!etag) throw new Error("uploadd failed");
			} catch (e) {
				setRootError("アップロードに失敗しました。再度お試しください");
				return;
			}

			const { data, error } = await client.PATCH("/me/avatar");

			if (error) return;

			useAuthStore.getState().setCurrentUser(data);
		}
	};

	const get_presigned_uri = async (media_type: string) => {
		const { data, error } = await client.POST("/me/avatar/presigned_uri", {
			body: { media_type: media_type },
		});

		if (error) {
			const errorCode: ErrorCode = error.code;
			switch (errorCode) {
				case "INVALID_MEDIA_TYPE":
					setRootError("png,jpeg,webpいずれかの画像にしてください");
				default:
					setRootError("エラーが発生しました。時間をおいて再度お試しください");
			}
			return;
		}
		const presigned_uri = data?.presigned_uri;
		return presigned_uri;
	};

	const updateUser = async (values: UpdateUserFormValues) => {
		const { data, error } = await client.PATCH("/me", {
			body: { display_name: values.display_name },
		});

		if (error) {
			const errorCode: ErrorCode = error.code;
			switch (errorCode) {
				case "INVALID_DISPLAY_NAME":
					setError("display_name", {
						message: "表示名を正しく入力してください",
					});
					break;
				case "RATE_LIMIT_EXCEEDED":
					setRootError("操作が多すぎます。時間をおいて再度お試しください");
					break;
				default:
					setRootError("エラーが発生しました。時間をおいて再度お試しください");
					break;
			}
			return;
		}

		useAuthStore.getState().setCurrentUser(data);
	};

	return (
		<View className="flex-1 bg-white px-7 justify-between">
			<View>
				<Pressable onPress={pickImage} className="items-center py-6 mt-6">
					<Avatar className="size-32 mb-2" alt="user avatar">
						<AvatarImage
							source={{ uri: current_user?.avatar_url ?? undefined }}
						/>
						<AvatarFallback className="bg-transparent">
							<Image
								source={require("@/assets/images/user_default_avatar.png")}
								style={{ width: "100%", height: "100%" }}
							/>
						</AvatarFallback>
					</Avatar>

					<Text className="text-gray-400 text-sm font-medium">変更する</Text>
				</Pressable>

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
				onPress={handleSubmit(updateUser)}
				size="lg"
				className="rounded-xl bg-main h-12 text-white mb-10"
				disabled={isSubmitting}
			>
				<Text className="text-white font-bold">
					{isSubmitting ? "更新中..." : "更新する"}
				</Text>
			</Button>
		</View>
	);
}
