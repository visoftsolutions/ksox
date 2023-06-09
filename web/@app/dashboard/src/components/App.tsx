import UserProfile from "./Profile/UserProfile";
export default function App() {
  return (
    <UserProfile
      styles="mt-28"
      name="Garek Majęcki"
      publicWallet="0x3acaDFB15E991e8403D2Fe3E75Ee4782B88cF5b1"
      badges={[
        { name: "", imgUrl: "gfx/placeholderBadge1.png", description: "" },
        { name: "", imgUrl: "gfx/placeholderBadge2.png", description: "" },
        { name: "", imgUrl: "gfx/placeholderBadge3.png", description: "" },
        { name: "", imgUrl: "gfx/placeholderBadge4.png", description: "" },
      ]}
    />
  );
}
