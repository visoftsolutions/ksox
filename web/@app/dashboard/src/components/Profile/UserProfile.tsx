import { createSignal, onMount, onCleanup } from "solid-js";
import UserProfileHorizontal from "./UserProfileVariations/UserProfileHorizontal";
import UserProfileVertical from "./UserProfileVariations/UserProfileVertical";

export interface UserProfileComponent {
  styles?: string;
  name: string;
  publicWallet: string;
  image?: string; // if not given then apply default image
  badgesImages: string[]; // can be later extended to badges objects with names/links on hover/click etc.
  maxBadges?: number; // in future you might want to limit the number of badges a user can highlight
}

export default function UserProfile(props: UserProfileComponent) {
  const [isMobile, setIsMobile] = createSignal(false);
  const [isMounted, setIsMounted] = createSignal(false);

  onMount(() => {
    setIsMobile(window.innerWidth <= 768);

    const handleResize = () => {
      setIsMobile(window.innerWidth <= 768);
    };

    window.addEventListener("resize", handleResize);
    setIsMounted(true);

    onCleanup(() => {
      window.removeEventListener("resize", handleResize);
    });
  });

  const renderUserProfile = () => {
    if (isMounted()) {
      return isMobile() ? <UserProfileVertical {...props} /> : <UserProfileHorizontal {...props} />;
    }
  };

  return (
    <div class="flex flex-col items-center">
      {/* couldn't reference window withour onMount(), because of that, for 0.4s we can see Horizontal version on mobile, I might add additional if to not render anything before onMount completes */}
      {/* {isMobile() ?
      <UserProfileVertical
        styles="mt-28"
        name="Garek Majęcki"
        publicWallet="0x3acaDFB15E991e8403D2Fe3E75Ee4782B88cF5b1"
        image="gfx/placeholderPhoto.jpeg"
        badgesImages={["gfx/placeholderBadge1.png", "gfx/placeholderBadge2.png", "gfx/placeholderBadge3.png", "gfx/placeholderBadge4.png"]}
      /> :
      <UserProfileHorizontal
        styles="mt-28"
        name="Garek Majęcki"
        publicWallet="0x3acaDFB15E991e8403D2Fe3E75Ee4782B88cF5b1"
        image="gfx/placeholderPhoto.jpeg"
        badgesImages={["gfx/placeholderBadge1.png", "gfx/placeholderBadge2.png", "gfx/placeholderBadge3.png", "gfx/placeholderBadge4.png"]}
      />} */}
      {renderUserProfile()} {/* this makes it to appear with a shor tdelay but doesn't "flicker" */}
    </div>
  );
}
