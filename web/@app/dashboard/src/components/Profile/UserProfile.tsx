import firstLastChars from "@web/utils/firstLastChars";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface UserProfileComponent {
  styles?: string;
  name: string;
  publicWallet: string;
  image?: string; // if not given then apply default image
  badgesImages: string[]; // can be later extended to badges objects with names/links on hover/click etc.
  maxBadges?: number; // in future you might want to limit the number of badges a user can highlight
}

export default function UserProfile(props: UserProfileComponent) {
    const copyToClipboard = (text: string): void => {
        navigator.clipboard.writeText(text);
      };
  return (
    <div class={`w-full max-w-7xl bg-gray-2 ${props.styles}`}>
      <div class="flex h-full flex-col lg:flex-row items-center">
        {/* in the future the way of passing image will propably change when they will be stored in db*/}
        <img
          src={props.image ? joinPaths(base, props.image) : joinPaths(base, "gfx/noPhotoPlaceholder.png")}
          alt="profile photo"
          class="m-8 h-48 w-48 rounded-full"
        />
        <div class="flex flex-col pb-8 items-center lg:items-start">
          {" "}
          {/* data without image */}
          <p class="pb-2 text-3xl lg:text-6xl font-extrabold">{props.name}</p>
          <div class="flex flex-row">
            
            {/* below two lines might be illegal  */}
            <p class="text-xl hidden lg:block">{props.publicWallet}</p>
            <p class="text-xl lg:hidden">{firstLastChars(props.publicWallet, 6, 6)}</p>

            <button onClick={() => copyToClipboard(props.publicWallet)} class="ml-1">
              <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512" fill="white" opacity={0.7}>
                <path d="M448 384H256c-35.3 0-64-28.7-64-64V64c0-35.3 28.7-64 64-64H396.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V320c0 35.3-28.7 64-64 64zM64 128h96v48H64c-8.8 0-16 7.2-16 16V448c0 8.8 7.2 16 16 16H256c8.8 0 16-7.2 16-16V416h48v32c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V192c0-35.3 28.7-64 64-64z" />
              </svg>
            </button>
          </div>
          {/* display badges */}
          <div class="flex flex-row">
          {props.maxBadges
              ? props.badgesImages.map((badge, idx) => {
                  if (idx < props.maxBadges!) return <img src={joinPaths(base, badge)} alt="badge" class="m-1 h-6 w-6" />;
                })
              : props.badgesImages.map((badge) => {
                  return <img src={joinPaths(base, badge)} alt="badge" class="m-1 h-6 w-6" />;
                })}
          </div>
        </div>
      </div>
    </div>
  );
}
