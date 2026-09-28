# 1-minute talk: Jupiter

Hard limit is 60 seconds. The script is ~110 spoken words (~50 s at a calm pace), which leaves a 10 s buffer.

| Time | Slide | Point |
|---|---|---|
| 0–12 s | 1 | What Jupiter is: the aggregator that finds the best price across all Solana DEXs |
| 12–27 s | 1 | Workflow (quote → sign → land → route) + why Solana |
| 27–35 s | 1 | Business model: 2–50 bps platform fee by pair |
| 35–50 s | 2 | Own on-chain programs + off-chain keepers |
| 50–60 s | 2 | Risks & improvements → takeaway |

## Spoken text

<!-- KA: spoken script -->
Jupiter Solana-ის მთავარი ლიკვიდობის აგრეგატორია. ლიკვიდობა ათობით DEX-ზეა გაფანტული, Jupiter კი ყველას ერთდროულად ამოწმებს და მომხმარებელს საუკეთესო ფასს ერთ ტრანზაქციაში აძლევს. მის API-ს უმეტესი Solana საფულე იყენებს.

პროცესი მარტივია: API აბრუნებს მზა ტრანზაქციას, მომხმარებელი მას საკუთარ საფულეში ხელს აწერს, ხოლო Jupiter-ის პროგრამა swap-ს რამდენიმე DEX-ში ატომურად ასრულებს. ეს Solana-ზე შესაძლებელია, რადგან საკომისიო იაფია და ერთ ტრანზაქციას ბევრი account-ის ჩართვა შეუძლია.

შემოსავალი swap-ის საკომისიოდან მოდის — წყვილის მიხედვით 0-დან 50 bps-მდე.

Phantom-ისგან განსხვავებით, Jupiter-ს საკუთარი onchain პროგრამები აქვს: swap, limit order, DCA და perps. თუმცა მათ შესრულებას offchain keeper-ები უზრუნველყოფენ.

მთავარი რისკებია upgradeable პროგრამები, oracle-ზე დამოკიდებულება და ცენტრალიზებული API. Mzia-სთვის კი Jupiter-ის API ხმით USDT-ზე კონვერტაციის საშუალებას იძლევა.
<!-- /KA -->

## Delivery tips
- Point at the **fee table** on slide 1, then switch to slide 2 at "Phantom-ისგან განსხვავებით…". That contrast with A3 is the strongest line.
- If you run long, cut the "ეს Solana-ზე შესაძლებელია…" sentence first. The slide already covers it.
- Rehearse once with a timer.
