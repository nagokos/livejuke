import { Image, Text, View, ScrollView, Pressable } from "react-native";

const POPULAR_SHOWS = [
	{ id: 1, artist: "Mr.Children", venue: "千葉", date: "4/4", votes: 342 },
	{ id: 2, artist: "King Gnu", venue: "東京", date: "4/10", votes: 256 },
	{ id: 3, artist: "あいみょん", venue: "大阪", date: "4/15", votes: 189 },
];

const UPCOMING_SHOWS = [
	{ id: 1, artist: "RADWIMPS", venue: "横浜", date: "3/28", daysLeft: 4 },
	{ id: 2, artist: "back number", venue: "名古屋", date: "3/30", daysLeft: 6 },
	{ id: 3, artist: "Vaundy", venue: "福岡", date: "4/2", daysLeft: 9 },
];

const POPULAR_ARTISTS = [
	{ id: 1, name: "Mr.Children", shows: 12 },
	{ id: 2, name: "King Gnu", shows: 8 },
	{ id: 3, name: "米津玄師", shows: 6 },
	{ id: 4, name: "あいみょん", shows: 5 },
	{ id: 5, name: "髭男", shows: 4 },
];

const RECENT_VOTES = [
	{ id: 1, artist: "藤井 風", venue: "大阪", date: "3/18", ago: "3分前" },
	{ id: 2, artist: "BUMP", venue: "東京", date: "3/19", ago: "10分前" },
];

function SectionHeader({
	title,
	onPress,
}: {
	title: string;
	onPress?: () => void;
}) {
	return (
		<View className="flex-row justify-between items-center mb-3 px-5">
			<Text className="text-lg font-semibold text-gray-900">{title}</Text>
			<Pressable onPress={onPress}>
				<Text className="text-xs text-[#534AB7]">すべて表示</Text>
			</Pressable>
		</View>
	);
}

export default function Home() {
	return (
		<View className="flex-1 bg-white">
			<ScrollView showsVerticalScrollIndicator={false}>
				<View className="pt-8 pb-4 px-5 flex-row justify-between items-center">
					<Text className="text-3xl font-bold text-gray-900">ホーム</Text>
					<Pressable className="w-9 h-9 rounded-full bg-gray-100 items-center justify-center">
						<Text className="text-gray-500 text-sm">🔍</Text>
					</Pressable>
				</View>
				{/* 盛り上がってるショー */}
				<View className="mb-6">
					<SectionHeader title="盛り上がってるショー" />
					<ScrollView
						horizontal
						showsHorizontalScrollIndicator={false}
						className="pl-5"
					>
						{POPULAR_SHOWS.map((show) => (
							<Pressable
								key={show.id}
								className="mr-3 w-36 rounded-xl overflow-hidden border border-gray-200"
							>
								<View className="h-24 bg-[#534AB7] items-center justify-center">
									<Text className="text-white text-2xl">♪</Text>
								</View>
								<View className="p-2.5">
									<Text className="text-sm font-semibold text-gray-900">
										{show.artist}
									</Text>
									<Text className="text-xs text-gray-400 mt-0.5">
										{show.venue} {show.date}
									</Text>
									<Text className="text-xs text-[#534AB7] mt-1">
										{show.votes}人が投票中
									</Text>
								</View>
							</Pressable>
						))}
					</ScrollView>
				</View>

				{/* もうすぐ開催 */}
				<View className="mb-6">
					<SectionHeader title="もうすぐ開催" />
					<ScrollView
						horizontal
						showsHorizontalScrollIndicator={false}
						className="pl-5"
					>
						{UPCOMING_SHOWS.map((show) => (
							<Pressable
								key={show.id}
								className="mr-3 w-36 rounded-xl bg-gray-50 p-3"
							>
								<Text className="text-sm font-semibold text-gray-900">
									{show.artist}
								</Text>
								<Text className="text-xs text-gray-400 mt-0.5">
									{show.venue} {show.date}
								</Text>
								<View className="mt-2 self-start bg-[#EEEDFE] rounded-md px-2 py-1">
									<Text className="text-xs text-[#3C3489]">
										あと{show.daysLeft}日
									</Text>
								</View>
							</Pressable>
						))}
					</ScrollView>
				</View>

				{/* 人気のアーティスト */}
				<View className="mb-6">
					<SectionHeader title="人気のアーティスト" />
					<ScrollView
						horizontal
						showsHorizontalScrollIndicator={false}
						className="pl-5"
					>
						{POPULAR_ARTISTS.map((artist) => (
							<Pressable key={artist.id} className="mr-4 items-center">
								<View className="w-14 h-14 rounded-full bg-[#534AB7]" />
								<Text className="text-xs text-gray-900 mt-1.5">
									{artist.name}
								</Text>
								<Text className="text-xs text-gray-400">{artist.shows}件</Text>
							</Pressable>
						))}
					</ScrollView>
				</View>

				{/* 最近投票されたショー */}
				<View className="mb-8 px-5">
					<SectionHeader title="最近投票されたショー" />
					{RECENT_VOTES.map((show) => (
						<Pressable
							key={show.id}
							className="flex-row items-center gap-3 p-3 bg-gray-50 rounded-xl mb-2"
						>
							<View className="w-10 h-10 rounded-lg bg-[#534AB7]" />
							<View className="flex-1">
								<Text className="text-sm font-semibold text-gray-900">
									{show.artist} - {show.venue} {show.date}
								</Text>
								<Text className="text-xs text-gray-400 mt-0.5">
									{show.ago}に投票あり
								</Text>
							</View>
						</Pressable>
					))}
				</View>
			</ScrollView>
		</View>
	);
}
