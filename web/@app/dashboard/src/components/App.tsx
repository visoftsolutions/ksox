import { createSignal, onCleanup, onMount } from "solid-js";
import UserProfileHorizontal from "./Profile/UserProfileVariations/UserProfileHorizontal";
import UserProfileVertical from "./Profile/UserProfileVariations/UserProfileVertical";
import UserProfile from "./Profile/UserProfile";

export default function App() {
  return (
    <UserProfile
      styles="mt-28"
      name="Garek Majęcki"
      publicWallet="0x3acaDFB15E991e8403D2Fe3E75Ee4782B88cF5b1"
      image="gfx/placeholderPhoto.jpeg"
      badgesImages={["gfx/placeholderBadge1.png", "gfx/placeholderBadge2.png", "gfx/placeholderBadge3.png", "gfx/placeholderBadge4.png"]}
    />
  );
}
