import { Link } from "expo-router";
import {
	KeyboardAvoidingView,
	Platform,
	Pressable,
	TextInput,
	View,
} from "react-native";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Lock, Mail, User, Eye, EyeOff } from "lucide-react-native";
import { useState } from "react";

export default function RegisterScreen() {
	const [showPassword, setShowPassword] = useState(false);
	return (
		<KeyboardAvoidingView
			className="flex-1 bg-white"
			behavior={Platform.OS === "ios" ? "padding" : "height"}
		>
			<View className="flex-1 justify-center px-8">
				<Text className="text-3xl mb-3 font-bold text-foreground">
					LiveJukeへようこそ！
				</Text>

				<View className="mt-8 gap-6">
					{/* 表示名 */}
					<View className="flex-row items-center gap-3 border-b border-primary/30 pb-3">
						<User size={20} className="text-primary" />
						<TextInput
							autoCapitalize="none"
							maxLength={30}
							placeholder="表示名"
							selectionColor="#534AB7"
							placeholderTextColor="#9CA3AF"
							className="flex-1 text-base text-foreground"
						/>
					</View>

					{/* メールアドレス */}
					<View className="flex-row items-center gap-3 border-b border-primary/30 pb-3">
						<Mail size={20} className="text-primary" />
						<TextInput
							placeholder="メールアドレス"
							placeholderTextColor="#9CA3AF"
							selectionColor="#534AB7"
							keyboardType="email-address"
							autoCapitalize="none"
							autoCorrect={false}
							autoComplete="email"
							textContentType="emailAddress"
							className="flex-1 text-base text-foreground"
						/>
					</View>

					{/* パスワード */}
					<View className="flex-row items-center gap-3 border-b border-primary/30 pb-3">
						<Lock size={20} className="text-primary" />
						<TextInput
							autoCapitalize="none"
							autoCorrect={false}
							autoComplete="new-password"
							textContentType="newPassword"
							maxLength={72}
							selectionColor="#534AB7"
							placeholder="パスワード"
							placeholderTextColor="#9CA3AF"
							className="flex-1 text-base text-foreground"
							secureTextEntry={!showPassword}
						/>
						<Pressable onPress={() => setShowPassword(!showPassword)}>
							{showPassword ? (
								<EyeOff size={20} className="text-primary" />
							) : (
								<Eye size={20} className="text-primary " />
							)}
						</Pressable>
					</View>
				</View>

				{/* 登録ボタン */}
				<Button className="mt-10 h-12 w-full rounded-xl bg-main">
					<Text className="font-bold">登録する</Text>
				</Button>

				{/* ログインリンク */}
				<View className="mt-6 flex-row items-center justify-center gap-1">
					<Text className="text-muted-foreground">
						アカウントをお持ちの方は
					</Text>
					<Link href="/login" asChild>
						<Button variant="link" size="sm" className="px-0">
							<Text className="text-main">ログイン</Text>
						</Button>
					</Link>
				</View>
			</View>
		</KeyboardAvoidingView>
	);
}
