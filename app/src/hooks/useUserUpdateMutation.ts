import { client } from "@/api/client";
import { AppErrorWithResponse } from "@/types/api";
import { components } from "@/types/schema";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import * as ImagePicker from "expo-image-picker";

type CurrentUserResponse = components["schemas"]["CurrentUserResponse"];
type UserUpdateParams = components["schemas"]["UserUpdateInput"];

export const useUserUpdateMutation = () => {
	const queryClient = useQueryClient();

	const refreshCurrentUser = async () => {
		await queryClient.invalidateQueries({
			queryKey: ["currentUser"],
		});
	};

	const updateUser = useMutation<
		CurrentUserResponse,
		AppErrorWithResponse,
		UserUpdateParams
	>({
		mutationFn: async (params) => {
			const { data, error, response } = await client.PATCH("/me", {
				body: params,
			});

			if (error) throw { error, response };
			return data;
		},
		onSuccess: refreshCurrentUser,
	});

	const pickImage = async () => {
		const permissionResult =
			await ImagePicker.requestMediaLibraryPermissionsAsync();

		if (!permissionResult.granted) {
			throw {
				error: {
					code: "MEDIA_FOLDER_DENIED",
					message: "media folder denied",
				},
			};
		}

		const result = await ImagePicker.launchImageLibraryAsync({
			mediaTypes: ["images"],
			allowsEditing: true,
			aspect: [1, 1],
			quality: 0.8,
		});

		return result;
	};

	const imageToBlob = async (image: ImagePicker.ImagePickerSuccessResult) => {
		const uri = image.assets[0]?.uri;
		if (!uri) throw new Error("upload failed");
		const uri_response = await fetch(uri);
		const blob = await uri_response.blob();
		return blob;
	};

	const uploadToS3 = async () => {
		const image = await pickImage();
		if (image.canceled) {
			throw {
				error: {
					code: "UPLOAD_CANCELED",
					message: "upload canceled",
				},
			};
		}

		const media_type = image.assets[0]?.mimeType;
		if (!media_type) {
			throw {
				error: {
					code: "INVALID_MEDIA_TYPE",
					message: "invalid media type",
				},
			};
		}

		const { data, error, response } = await client.POST(
			"/me/avatar/presigned_uri",
			{
				body: { media_type },
			},
		);
		if (error) throw { error, response };

		try {
			const blob = await imageToBlob(image);

			const result = await fetch(data.presigned_uri, {
				method: "PUT",
				headers: { "Content-Type": media_type },
				body: blob,
			});

			const etag = result.headers.get("etag");
			if (!etag) throw new Error("upload failed");
		} catch {
			throw {
				error: {
					code: "UPLOAD_FAILED",
					message: "upload failed",
				},
			};
		}
	};

	const updateAvatar = useMutation<CurrentUserResponse, AppErrorWithResponse>({
		mutationFn: async () => {
			await uploadToS3();
			const { data, error, response } = await client.PATCH("/me/avatar");
			if (error) throw { error, response };
			return data;
		},
		onSuccess: refreshCurrentUser,
	});

	return {
		updateUser,
		updateAvatar,
		isProcessing: updateUser.isPending || updateAvatar.isPending,
	};
};
