import { DownloadIcon } from "../Icons/download";
import { GearIcon } from "../Icons/gear";
import { TrashIcon } from "../Icons/trash";


export const TitleScreen = () => {
    return <div class="flex min-h-full">
        <div class="p-10 bg-[#E4DCCF] flex flex-col gap-28">
            <div>
                <img src="./assets/images/book.png" height="164px" width="152px" />
                <p class="text-[40px]">Storytell</p>
            </div>
            <div class="flex flex-col gap-[38px]">
                <div class="bg-[#F0EBE3] p-1 mr-[-90px] cursor-pointer">
                    <p class="text-[20px]">Stories</p>
                </div>
                <div>
                    <p class="text-[20px] p-1 cursor-pointer">Settings</p>
                </div>
                <div>
                    <p class="text-[20px] p-1 cursor-pointer">Guides</p>
                </div>
            </div>
        </div>
        <div class="w-full pt-[24px] pl-[32px] flex flex-col gap-28">
            <div class="flex gap-8">
                <div class="rounded-lg bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition">
                    <p>Create</p>
                </div>
                <div class="rounded-lg bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition">
                    <p>Import</p>
                </div>
            </div>
            <div class="w-full flex flex-col gap-12">
                <div class="w-[50vw] h-[89px] bg-[#E4DCCF] drop-shadow-lg rounded-lg flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] hover:scale-[1.008] transition">
                    <div class="flex justify-between items-center">
                        <p class="text-[20px]">Story #1</p>
                        <div class="flex gap-4">
                            <GearIcon />
                            <DownloadIcon />
                            <TrashIcon />
                        </div>
                    </div>
                    <div>
                        <p class="text-[14px]">This is the description of the story...</p>
                    </div>
                </div>
                <div class="w-[50vw] h-[89px] bg-[#E4DCCF] drop-shadow-lg rounded-lg flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] hover:scale-[1.008] transition">
                    <div class="flex justify-between items-center">
                        <p class="text-[20px]">Story #2</p>
                        <div class="flex gap-4">
                            <GearIcon />
                            <DownloadIcon />
                            <TrashIcon />
                        </div>
                    </div>
                    <div>
                        <p class="text-[14px]">This is the description of the story...</p>
                    </div>
                </div>
                <div class="w-[50vw] h-[89px] bg-[#E4DCCF] drop-shadow-lg rounded-lg flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] hover:scale-[1.008] transition">
                    <div class="flex justify-between items-center">
                        <p class="text-[20px]">Story #3</p>
                        <div class="flex gap-4">
                            <GearIcon />
                            <DownloadIcon />
                            <TrashIcon />
                        </div>
                    </div>
                    <div>
                        <p class="text-[14px] truncate">This is the description of the story, in fact this is a veeeeery long description that is going to bore you... seriously stop reading it and never click on this story!</p>
                    </div>
                </div>
            </div>
        </div>
    </div>;
};