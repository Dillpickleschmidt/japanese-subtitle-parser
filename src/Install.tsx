import { createSignal } from "solid-js";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "./components/ui/button";
import { invoke } from "@tauri-apps/api/tauri";

export default function Install() {
  const [expandedItem, setExpandedItem] = createSignal(["item-0"]);
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
              <span class="font-semibold text-lg">Step 1- Install Ichiran</span>
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
              Use Ichiran to parse every transcript file that you have. This may
              take a while.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-3">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 3- Copy Database</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              Copy the Ichiran postgres database to sqlite so that it can be
              used by the search engine.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-4">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 4- Create Indexes</span>
            </AccordionTrigger>
            <AccordionContent class="text-base">
              Create indexes so you can search the database quickly.
            </AccordionContent>
          </AccordionItem>
          <AccordionItem value="item-5">
            <AccordionTrigger>
              <span class="text-lg font-semibold">Step 5- Start Searching</span>
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
                TLDR {"->"} Organize transcript files, send them to Ichiran,
                parse in Ichiran, put the words in database, then search at your
                convenience.
              </p>
            </div>
          )}
          {expandedItem().includes("item-1") && (
            <div class="space-y-3">
              <h1 class="text-center font-extrabold text-xl pb-4">
                Install Ichiran
              </h1>
              <p>
                Ichiran is the Japanese text parser. It tokenizes Japanese text
                (splitting up unique words), and gives us a number pointing to
                the ID of the unconjugated version for each word in the JMDict
                dictionary (which gets installed with Ichiran). This lets you
                not have to worry about conjugations when searching.
              </p>
              <p>
                There are other Japanese text parsers out there, but Ichiran
                outperforms everything else in terms of quality. You can test it
                out on{" "}
                <a
                  href="https://ichi.moe/"
                  target="_blank"
                  class="text-sky-500 hover:cursor-pointer hover:underline"
                >
                  Ichi.Moe
                </a>
                .
              </p>
              <p>
                Follow the installation process{" "}
                <a
                  href="https://readevalprint.tumblr.com/post/639359547843215360/ichiranhome-2021-the-ultimate-guide"
                  target="_blank"
                  class="text-sky-500 hover:cursor-pointer hover:underline"
                >
                  here
                </a>
                . It's a little complicated, but it's critical for the search
                engine to work. Run the test suite, and if everything passes,
                move on to the next step.
              </p>
              <ol class="list-decimal list-inside">
                {/* <li>
                  Download the .pgdump file from the{" "}
                  <a
                    href="https://github.com/tshatrov/ichiran/releases"
                    target="_blank"
                    class="text-sky-500 hover:cursor-pointer hover:underline"
                  >
                    latest release of Ichiran
                  </a>
                  . This is a database backup file that we're going to restore
                </li>
                <li>Test 2</li> */}
              </ol>
            </div>
          )}
          {expandedItem().includes("item-2") && (
            <div class="space-y-3 pb-12">
              <h1 class="text-center font-extrabold text-xl pb-4">
                Parse Subtitles
              </h1>
              <p>
                The next step is to add a custom LISP script for batch parsing
                subtitle files.
              </p>
              <p>
                We're not going to use the Ichiran CLI tool since it's limited
                to a single core and we have a lot of files to parse. Instead,
                we're going to run Ichiran directly with a custom LISP script
                that comes with this project.
              </p>
              <p>
                Find the directory ~/quicklisp/local-projects
                (%USERPROFILE%\quicklisp\local-projects on Windows). You should
                see the ichiran folder there. Paste the reverse-index-ichiran
                folder included in this project next to it.
              </p>
              <p>
                Find the subtitles you want place them in a folder. Each show
                should have its own folder within. For example:
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
                Select the root, source directory of your transcripts. Select
                the destination directory where you want to save the
                transcripts.csv file (this should be in the
                reverse-index-ichiran folder). Then, press the "Parse" button.
              </p>
              <div class="flex justify-center gap-6 py-4">
                <Button variant="outline">Select Source</Button>
                <Button variant="outline">Select Destination</Button>
                <Button
                  variant="outline"
                  onClick={async () => {
                    try {
                      const result = await invoke("process_srt_directory", {
                        rootDir: "data/transcripts_raw",
                      });
                      console.log("Process result:", result);
                      // Handle successful result (e.g., show a success message)
                    } catch (error) {
                      console.error("Error processing SRT files:", error);
                      // Handle error (e.g., show an error message to the user)
                    }
                  }}
                >
                  Parse
                </Button>
              </div>
              <p>
                Open your terminal and cd into the reverse-index-ichiran folder.
                Then, run the following commands in your terminal:
              </p>
              <ol class="list-decimal list-inside space-y-2 ml-3">
                <li class="!-mt-1">
                  sbcl --dynamic-space-size 32768 # Set this to some value less
                  than the total amount of RAM that you have in MB.
                </li>
                <li>(load "ichiran-file-processor.lisp")</li>
                <li>(ichiran-file-processor:main "transcripts.csv")</li>
              </ol>
              <p>This may take a while.</p>
            </div>
          )}
          {expandedItem().includes("item-4") && (
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
                onClick={() => invoke("create_reverse_index")}
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
