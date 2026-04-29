import { useQuery } from "@tanstack/react-query";
import { client } from "@/api/client";
import { useAuthStore } from "@/stores/auth";

export const useAuthQuery = () => {
	const hasToken = useAuthStore((state) => state.hasToken);

	return useQuery({
		queryKey: ["currentUser"],
		queryFn: async () => {
			const { data, error } = await client.GET("/me");
			if (error) {
				return null;
			}
			return data;
		},
		staleTime: Infinity,
		retry: false,
		enabled: hasToken,
	});
};

export const useCurrentUser = () => {
	const { data, ...rest } = useAuthQuery();
	return { ...rest, currentUser: data?.user ?? null };
};

export const useAuthStatus = () => {
	const { data, ...rest } = useAuthQuery();
	return { ...rest, authStatus: data?.auth_status ?? null };
};
