import "./App.css";

import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { Display } from "./types/ScreenshotTypes";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [image, setImage] = useState("");

  const [screens, setScreens] = useState<Display>([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function screenshot() {
    const screenshot = await invoke("capture_screenshot");

    setImage(screenshot as string);
  }

  const getScreens = async () => {
    const screens = await invoke("get_available_screens");

    const screensArray = [];

    for (const screen of JSON.parse(screens)) {
      screensArray.push({
        ...screen,
        isPrimary: screen.is_primary,
      });
    }

    setScreens(screensArray as Display[]);
  };

  return (
    <div className="bg-slate-900 min-h-screen flex flex-col items-center justify-center text-white p-8">
      <div className="container center">
        <h1>Welcome to Eevie!</h1>

        {image && <img src={image} />}

        <div>
          <button onClick={screenshot}>Capture Screenshot</button>
        </div>

        <div className="row">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <button type="submit">Greet</button>
          </form>
        </div>
        <div>
          <button onClick={getScreens}>Get Screens</button>
        </div>
        <p>{greetMsg}</p>
      </div>

      <h4>Available Screens</h4>
      <p>{JSON.stringify(screens, null, 2)}</p>
    </div>
  );
}

export default App;
