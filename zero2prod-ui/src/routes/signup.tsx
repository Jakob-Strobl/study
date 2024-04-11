export default function SignupField() {
  return (
    <div class="flex h-screen flex-col items-center">
      <h1>Signup to my Newsletter</h1>
      <div class="w-3/12 flex flex-col items-center gap-4 p-12 rounded-2xl bg-slate-400">
        <h2 class="text-lg font-medium">Just fill in the fields below</h2>
        <label class="flex w-10/12 bg-slate-300 p-2 rounded-lg">
          <span class="w-14">Email:</span>
          <input class="w-full ml-2 px-1" placeholder="Enter your email adress."></input>
        </label>
        <label class="flex w-10/12 bg-slate-300 p-2 rounded-lg">
          <span class="w-14">Name:</span>
          <input class="w-full ml-2 px-1" placeholder="Enter your name/alias"></input>
        </label>
        <button class="w-1/4 bg-slate-800 p-2 rounded-md text-white mt-2">Signup</button>
      </div>
      <div class="flex flex-col gap-0 m-2">
        <p class="m-2">I promise. I won't sell your data. This is purely for educational purposes.</p>
        <p class="m-2 font-light">This will not run more than a few minutes at most. Sadge.</p>
      </div>
    </div>
  )
}