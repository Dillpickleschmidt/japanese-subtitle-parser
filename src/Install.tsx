import { createSignal } from "solid-js";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "./components/ui/button";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

export default function Install() {
  const [expandedItem, setExpandedItem] = createSignal(["item-0"]);
  const [selectedDirectory, setSelectedDirectory] = createSignal<string>("");
  const [isProcessing, setIsProcessing] = createSignal(false);

  const selectDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Subtitle Directory",
      });

      if (selected && typeof selected === "string") {
        setSelectedDirectory(selected);
      }
    } catch (error) {
      console.error("Error selecting directory:", error);
    }
  };

  const processSubtitles = async () => {
    if (!selectedDirectory()) {
      alert("Please select a directory first!");
      return;
    }

    setIsProcessing(true);
    try {
      const result = await invoke("import_subtitles_from_directory", {
        rootDir: selectedDirectory(),
      });
      console.log("Process result:", result);
      alert("Processing completed successfully!");
    } catch (error) {
      console.error("Error processing SRT files:", error);
      alert(`Error: ${error}`);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div class="relative 2xl:px-32 xl:px-12 px-6 py-12 h-screen flex flex-col items-center text-lg">
      <h1 class="text-3xl 2xl:text-4xl font-bold">Installation Steps</h1>
      <div class="relative 2xl:mt-16 mt-10 h-full w-full">
        <Accordion
          collapsible
          class="w-1/3 pb-24 ml-20 h-full flex flex-col justify-center"
          value={expandedItem()}
          onChange={setExpandedItem}
        >
          <AccordionItem value="item-0">
            <AccordionTrigger>
              <span class="font-semibold text-lg">Introduction</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              This tool provides a nicer UI than a CLI for indexing and
              searching transcripts. However, it still requires a{" "}
              <span class="text-sky-500 hover:cursor-pointer hover:underline">
                multi-step
              </span>{" "}
              installation process.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-1">
            <AccordionTrigger>
              <span class="font-semibold text-lg">Step 1- Install Kagome</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              Follow the installation instructions from the{" "}
              <a
                href="https://readevalprint.tumblr.com/post/639359547843215360/ichiranhome-2021-the-ultimate-guide"
                target="_blank"
                class="text-sky-500"
              >
                Ichiran guide
              </a>
              , or follow this quick guide:
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-2">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 2- Parse Subtitles</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              Parse subtitle files and create the database. This process uses
              kagome for Japanese morphological analysis and may take a while
              depending on the number of files.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-3">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 3- Create Indexes</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              Create reverse indexes to enable fast word searching across all
              subtitle files.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-4">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 4- Start Searching</span>
            </AccordionTrigger>
          </AccordionItem>
        </Accordion>
        <div class="absolute w-[55%] right-0 bottom-0 rounded-sm min-h-[500px] h-full p-12 bg-neutral-600/10 border border-muted text-base overflow-y-auto">
          {expandedItem().includes("item-0") && (
            <div class="space-y-3">
              <h1 class="text-center font-extrabold text-xl pb-4">
                How It Works
              </h1>
              <ol class="list-decimal list-inside space-y-2">
                <li>
                  Download .srt subtitle files from your favorite shows via{" "}
                  <a
                    href="https://jimaku.cc/"
                    target="_blank"
                    class="text-sky-500 hover:cursor-pointer hover:underline"
                  >
                    Jimaku
                  </a>{" "}
                  or any other source.
                </li>
                <li>
                  Read all the files you've downloaded list each transcript in a
                  transcripts.csv file.
                </li>
                <li>
                  Parse each transcript in that that{" "}
                  <strong>transcripts.csv</strong> file using{" "}
                  <a
                    href="https://github.com/tshatrov/ichiran"
                    target="_blank"
                    class="text-sky-500 hover:cursor-pointer hover:underline"
                  >
                    Ichiran
                  </a>
                  . This will tokenize (split words) from transcript sentences
                  and output a list of words and their IDs for each transcript
                  in a <strong>parsed_transcripts.csv</strong> file.
                </li>
                <li>
                  Add the words from the <strong>parsed_transcripts.csv</strong>{" "}
                  file to a database for quick and easy searching.
                </li>
                <li>
                  Search whichever Japanese word you want and it will return all
                  instances where that word occurs.
                </li>
              </ol>
              <p>
                TLDR {"->"} Organize subtitle files, parse them with kagome,
                store words in database, then search at your convenience.
              </p>
            </div>
          )}
          {expandedItem().includes("item-1") && (
            <div class="space-y-3">
              <h1 class="text-center font-extrabold text-xl pb-4">
                Install Kagome
              </h1>
              <p>
                Kagome is a fast Japanese morphological analyzer written in Go.
                It tokenizes Japanese text (splitting up unique words) and
                provides readings for each word. This eliminates the need for
                complex external dependencies.
              </p>
              <p>
                First, make sure you have Go installed on your system. Then
                install kagome:
              </p>
              <div class="mt-2 p-3 bg-gray-100 dark:bg-gray-800 rounded-md font-mono text-sm">
                go install github.com/ikawaha/kagome/v2@latest
              </div>
              <p class="mt-2">Verify the installation by running:</p>
              <div class="mt-2 p-3 bg-gray-100 dark:bg-gray-800 rounded-md font-mono text-sm">
                kagome --help
              </div>
              <p class="mt-2">
                If you see the help output, kagome is installed correctly and
                ready to use.
              </p>
            </div>
          )}
          {expandedItem().includes("item-2") && (
            <div class="space-y-3 pb-12">
              <h1 class="text-center font-extrabold text-xl pb-4">
                Parse Subtitles
              </h1>
              <p>
                Place your subtitle files in the appropriate folder structure.
                Each show should have its own folder with subtitle files inside:
              </p>
              <div class="ml-3">
                TranscriptsFolder
                <div class="ml-6">
                  Show1 <br />
                  <div class="ml-6">
                    Episode1.srt <br />
                  </div>
                  <div class="ml-6">
                    Episode2.srt <br />
                  </div>
                </div>
                <div class="ml-6">
                  Show2 <br />
                  <div class="ml-6">
                    Episode1.srt <br />
                  </div>
                  <div class="ml-6">
                    Episode2.srt <br />
                  </div>
                </div>
              </div>
              <p class="!mt-4">
                Select your subtitle directory and click "Parse Subtitles" to
                process all files. This will automatically use kagome to analyze
                the Japanese text and extract words.
              </p>

              {selectedDirectory() && (
                <div class="mt-3 p-3 bg-green-100 dark:bg-green-900 rounded-md">
                  <strong>Selected Directory:</strong>
                  <br />
                  <span class="font-mono text-sm">{selectedDirectory()}</span>
                </div>
              )}

              <div class="flex justify-center gap-4 py-4">
                <Button variant="outline" onClick={selectDirectory}>
                  Select Directory
                </Button>
                <Button
                  variant="outline"
                  onClick={processSubtitles}
                  disabled={!selectedDirectory() || isProcessing()}
                >
                  {isProcessing() ? "Processing..." : "Parse Subtitles"}
                </Button>
              </div>
              <p>
                This process may take a while depending on the number of
                subtitle files.
              </p>
            </div>
          )}
          {expandedItem().includes("item-3") && (
            <div class="space-y-3">
              <h1 class="text-center font-extrabold text-xl pb-4">
                Create Reverse Index
              </h1>
              <p>
                Click the button to generate reverse indexes. This may take a
                while.
              </p>
              <Button
                variant="outline"
                onClick={async () => {
                  try {
                    const result = await invoke("analyze_japanese_transcripts");
                    console.log("Index result:", result);
                    alert("Reverse index created successfully!");
                  } catch (error) {
                    console.error("Error creating reverse index:", error);
                    alert(`Error: ${error}`);
                  }
                }}
              >
                Create Reverse Index
              </Button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
