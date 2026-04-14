import { View, Text, TextInput } from "react-native";
import { Button } from "@/components/ui/button";

export default function EmailEdit() {
	return (
		<View className="flex-1 bg-white px-6 justify-between">
			<View className="mt-10 gap-6">
				<View className="gap-1">
					<Text className="text-sm text-gray-500">現在のメールアドレス</Text>
					<Text className="text-base font-medium">kosuda0428@gmail.com</Text>
				</View>

				<View className="gap-2 mt-2">
					<Text className="text-sm font-medium">新しいメールアドレス</Text>

					<TextInput
						autoCapitalize="none"
						autoCorrect={false}
						keyboardType="email-address"
						className="h-12 rounded-lg border border-gray-200 bg-gray-50 px-4"
						placeholder="example@email.com"
					/>
				</View>
			</View>

			<Button size="lg" className="rounded-xl h-12 bg-main mb-10">
				<Text className="text-white font-bold">メールアドレスを更新</Text>
			</Button>
		</View>
	);
}
