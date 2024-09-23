/* @refresh reload */
import { render } from "solid-js/web";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { A, Route, Router } from "@solidjs/router";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemControl,
  RadioGroupItemLabel,
} from "@/components/ui/radio-group";
import { For } from "solid-js";
import { Button } from "@/components/ui/button";

import Install from "./Install";
import Search from "./Search";

import "./App.css";

const App = (props: any) => (
  <>
    <div class="absolute bottom-12 left-12 z-10">
      <div class="opacity-50">
        <RadioGroup defaultValue="Install" class="grid gap-2">
          <For
            each={[
              "First Time Install",
              "Add New Transcripts",
              "Search",
              "Info",
            ]}
          >
            {(page) => (
              <RadioGroupItem value={page} class="flex items-center gap-2">
                <RadioGroupItemControl />
                <RadioGroupItemLabel class="text-base hover:underline">
                  <A
                    href={
                      page.toLowerCase() === "first time install"
                        ? "/"
                        : `/${page.toLowerCase()}`
                    }
                  >
                    {page}
                  </A>
                </RadioGroupItemLabel>
              </RadioGroupItem>
            )}
          </For>
        </RadioGroup>
      </div>
      <Button size="sm" variant="outline" class="mt-2 opacity-75">
        Set Default
      </Button>
    </div>
    {props.children}
  </>
);

render(
  () => (
    <>
      <ColorModeScript />
      <ColorModeProvider>
        <Router root={App}>
          <Route path="/" component={Install} />
          <Route path="/search" component={Search} />
        </Router>
      </ColorModeProvider>
    </>
  ),
  document.getElementById("root") as HTMLElement
);
