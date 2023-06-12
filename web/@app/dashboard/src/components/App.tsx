import Badges from "./Badges";
import Profile from "./Profile";

export default function App() {
  return (
    <div class="m-auto grid max-w-2xl grid-rows-[auto_1fr] items-center justify-stretch gap-10 md:max-w-7xl">
      <Profile name={"Okm165"} publicWallet={"0x21B17b25b864659F2947a80B2A8E36f372f1D945"} badges={[
        {name: "Crypto Connoisseur"},
        {name: "Crypto Connoisseur"},
        {name: "Crypto Connoisseur"},
      ]} />
      <Badges
        badges={[
          {
            name: "Crypto Connoisseur",
            description: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.",
            progress: 0.7,
          },
          {
            name: "Crypto Veteran",
            description: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.",
            progress: 0.3,
          },
          {
            name: "Crypto Overlord",
            description: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.",
            progress: 0.1,
          },
          {
            name: "Cryptonaut Forever",
            description: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.",
            progress: 0.9,
          },
          {
            name: "Cryptonaut Forever",
            description: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.",
            progress: 0.99,
          },
        ]}
      />
    </div>
  );
}
