

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
                <div>
                    <p class="text-[20px] p-1 cursor-pointer">Exit</p>
                </div>
            </div>
        </div>
        <div class="w-[100vw] bg-[#F0EBE3]">
            
        </div>
    </div>
}