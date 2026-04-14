import { View, Text, Pressable, TextInput, Image } from "react-native";
import * as ImagePicker from "expo-image-picker";
import { useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";

export default function ProfileEdit() {
	const [image, setImage] = useState<string | null>(null);

	const pickImage = async () => {
		const permissionResult =
			await ImagePicker.requestMediaLibraryPermissionsAsync();

		if (!permissionResult.granted) {
			alert("写真へのアクセスが許可されていません");
			return;
		}

		// 画像選択
		const result = await ImagePicker.launchImageLibraryAsync({
			mediaTypes: ["images"],
			allowsEditing: true,
			aspect: [1, 1],
			quality: 0.8,
		});

		if (!result.canceled) {
			setImage("https://github.com/mrzachnugent.png");
		}
	};

	return (
		<View className="flex-1 bg-white px-7 justify-between">
			{/* 上コンテンツ */}
			<View>
				<Pressable onPress={pickImage} className="items-center py-6 mt-6">
					{image ? (
						<Image
							source={{ uri: image }}
							className="size-24 rounded-full mb-2"
						/>
					) : (
						<Avatar className="size-24 mb-2" alt="avatar">
							<AvatarImage
								source={{ uri: "https://github.com/mrzachnugent.png" }}
							/>
							<AvatarFallback>
								<Text>ZN</Text>
							</AvatarFallback>
						</Avatar>
					)}

					<Text className="text-gray-400 text-sm font-medium">変更する</Text>
				</Pressable>

				<View className="gap-2">
					<Text className="text-sm font-medium text-foreground">表示名</Text>

					<TextInput
						autoCapitalize="none"
						autoCorrect={false}
						className="h-12 rounded-lg border border-gray-200 bg-gray-50 pl-4 pr-10"
						placeholder="表示名を入力"
					/>
				</View>
			</View>

			<Button size="lg" className="rounded-xl bg-main h-12 text-white mb-10">
				<Text className="text-white font-bold">更新する</Text>
			</Button>
		</View>
	);
}
