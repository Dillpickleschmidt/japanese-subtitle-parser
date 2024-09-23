import { TextField, TextFieldRoot } from "@/components/ui/textfield";
import { createEffect, createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "./components/ui/button";
import {
  Checkbox,
  CheckboxControl,
  CheckboxLabel,
} from "./components/ui/checkbox";

export default function Search() {
  const [value, setValue] = createSignal("");
  const [shows, setShows] = createSignal<
    Array<{ id: number; name: string; checked: boolean }>
  >([]);
  const [searchResults, setSearchResults] = createSignal("");

  // Fetch shows when component mounts
  createEffect(async () => {
    const allShows = await invoke<Array<[number, string]>>("get_all_shows");
    console.log(allShows);
    setShows(allShows.map(([id, name]) => ({ id, name, checked: true })));
  });

  const onSubmit = async (e: Event) => {
    e.preventDefault();
    const enabledShows = shows()
      .filter((show) => show.checked)
      .map((show) => show.id);
    try {
      const results = await invoke<string>("search_word_with_context", {
        word: value(),
        enabledShowIds: enabledShows,
      });
      setSearchResults(results);
    } catch (error) {
      console.error("Error during search:", error);
    }
  };

  const toggleShow = (id: number) => {
    setShows((shows) =>
      shows.map((show) =>
        show.id === id ? { ...show, checked: !show.checked } : show
      )
    );
  };

  const toggleAllShows = () => {
    const anyChecked = shows().some((show) => show.checked);
    setShows((shows) =>
      shows.map((show) => ({ ...show, checked: !anyChecked }))
    );
  };

  return (
    <div class="relative px-6 py-12 min-h-screen text-lg">
      <h1 class="text-3xl 2xl:text-4xl font-bold mt-12 text-center">
        Japanese Subtitle Search
      </h1>
      <div class="2xl:mt-16 mt-10 w-full flex">
        <div class="w-3/4 2xl:ml-72 xl:ml-56">
          <form onSubmit={onSubmit} class="flex">
            <TextFieldRoot class="w-full mx-auto">
              <TextField
                type="text"
                placeholder="Search a single word or full sentence, conjugations are handled automatically..."
                class="placeholder:text-base text-lg py-6 px-6"
                value={value()}
                onInput={(e) => setValue(e.currentTarget.value)}
              />
            </TextFieldRoot>
            <Button
              type="submit"
              size="lg"
              variant="secondary"
              class="ml-2 py-6 px-6"
            >
              Submit
            </Button>
          </form>
          <div class="rounded-sm min-h-[600px] bg-neutral-600/10 border border-muted mt-6 p-4 overflow-auto">
            {searchResults()}
          </div>
        </div>
        <div class="w-1/4 pl-4">
          <div class="flex justify-between items-center mb-4">
            <h2 class="text-xl font-bold">Shows</h2>
            <Button onClick={toggleAllShows} size="sm" variant="outline">
              Toggle All
            </Button>
          </div>
          <For each={shows()}>
            {(show) => (
              <div class="flex items-center mb-2">
                <Checkbox
                  id={`show-${show.id}`}
                  checked={show.checked}
                  onChange={() => toggleShow(show.id)}
                  class="flex items-start space-x-2 hover:cursor-pointer"
                >
                  <CheckboxControl />
                  <div class="grid gap-1.5 leading-none">
                    <CheckboxLabel class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 hover:cursor-pointer">
                      {show.name}
                    </CheckboxLabel>
                  </div>
                </Checkbox>
              </div>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
