export default function SignupField() {
  return (
    <div class="flex h-screen flex-col items-center justify-center">
      <div class="w-5/12 flex flex-col items-center gap-4">
        <label class="bg-slate-200 p-2">
          email:
          <input class="ml-2 px-1" placeholder="Enter your email adress."></input>
        </label>
        <label class="bg-slate-200 p-2">
          name:
          <input class="ml-2 px-1" placeholder="Enter your name/alias"></input>
        </label>
      </div>
    </div>
  )
}