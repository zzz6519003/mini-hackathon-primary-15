import dynamic from "next/dynamic";

const App = dynamic(() => import("../components/app"), {
  ssr: false,
});

export default function Home() {
  return (
    <div>
      <App/>  
    </div>
  );
}
