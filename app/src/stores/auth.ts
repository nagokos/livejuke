import { create } from "zustand";
import { clearAuth } from "@/lib/auth-storage";
import { queryClient } from "@/lib/query-client";
import { useCurrentUser } from "@/hooks/useCurrentUser";

interface AuthState {
	hasToken: boolean;
	setHasToken: () => void;
	logout: () => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
	hasToken: false,
	setHasToken: () => set({ hasToken: true }),

	logout: async () => {
		try {
			await clearAuth();
		} catch (error) {
			console.error("トークンの削除に失敗しました:", error);
		} finally {
			queryClient.clear();
			set({ hasToken: false });
		}
	},
}));

export const useIsLoggedIn = () => {
	const { currentUser, isSuccess } = useCurrentUser();
	return { isLoggedIn: !!currentUser, isSuccess };
};
