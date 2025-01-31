import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block.ts";

describeSuite({
  id: "D0515",
  title: "Configuration - ActiveConfig - MaxCollators",
  foundationMethods: "dev",
  testCases: ({ context, log, it }) => {
    let alice;
    beforeAll(async function () {
      const keyring = new Keyring({ type: "sr25519" });
      alice = keyring.addFromUri("//Alice", { name: "Alice  default" });

      const config = await context
        .polkadotJs()
        .query.configuration.activeConfig();
      expect(config["maxCollators"].toString()).toBe("100");

      const { result } = await context.createBlock(
        context
          .polkadotJs()
          .tx.sudo.sudo(
            context.polkadotJs().tx.configuration.setMaxCollators(200)
          )
          .signAsync(alice)
      );
      expect(result!.successful, result!.error?.name).to.be.true;

      await jumpSessions(context, 2);
    });

    it({
      id: "T01",
      title: "should set max collators after 2 sessions",
      test: async function () {
        const config = await context
          .polkadotJs()
          .query.configuration.activeConfig();
        expect(config["maxCollators"].toString()).toBe("200");
      },
    });
  },
});
