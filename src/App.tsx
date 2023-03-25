import "./App.css";

import { invoke } from "@tauri-apps/api/tauri";
import {
  isRegistered,
  register,
  unregister,
} from "@tauri-apps/api/globalShortcut";
import { useEffect, useState } from "react";
import { Display } from "./types/ScreenshotTypes";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [image, setImage] = useState("");

  const [screens, setScreens] = useState<Display[]>([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const baseShortCut = "CmdOrCtrl+Shift+Alt+";

  const registerShortcut = async () => {
    if (screens.length == 0) {
      const displays = await invoke("getDisplays");
      setScreens(displays);
    }

    console.log(screens);

    screens.forEach(async (screen, index) => {
      const key = baseShortCut + (index + 1);
      const alreadyRegistered = await isRegistered(key);
      if (alreadyRegistered) {
        await unregister(key);
      }
      await register(key, async () => {
        console.log(`Shortcut ${key} pressed!`);
        const screenshot = await invoke("capture_screenshot", {
          id: screen.id.toString(),
        });
        setImage(screenshot);
      });
    });
  };

  async function screenshot(id: string) {
    console.log(screens);
    if (screens.length === 0) {
      return;
    }

    const screenshot = await invoke("capture_screenshot", {
      id: id.toString(),
    });

    setImage(screenshot as string);
  }

  const getScreens = async () => {
    const screens = await invoke("get_available_screens");

    const screensArray = [];

    for (const screen of JSON.parse(screens as string)) {
      screensArray.push({
        ...screen,
        isPrimary: screen.is_primary,
      });
    }

    setScreens(screensArray as Display[]);
  };

  useEffect(() => {
    registerShortcut();
  }, [screens]);

  return (
    <div className="bg-slate-900 min-h-screen flex flex-col items-center justify-center text-white p-8">
      <div className="container center">
        <h1>Welcome to Eevie!</h1>

        {image && <img src={image} />}

        <div></div>

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
      {screens.map((screen) => (
        <div key={screen.id}>
          <p>{JSON.stringify(screen, null, 2)}</p>
          <button onClick={() => screenshot(screen.id)}>Capture</button>
        </div>
      ))}
    </div>
  );
}

export default App;
