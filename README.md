<h1 align="center">
  <br>
  <img src="https://github.com/odoo/odoo-vscode/blob/release/images/odoo_logo.png?raw=true"></a>
  <br>
  Odoo Zed Extension
  <br>
</h1>

<h4 align="center">Boost your Odoo code development</h4>

## About

This extension integrates the Odoo Language Server, that will help you in the development of your Odoo projects.

This repository contains the code that build the Zed extension for OdooLS. OdooLs itself is available [here](https://github.com/odoo/odoo-ls)

## Settings

You can provide settings to the plugin:

```json
  "lsp": {
    "odoo": {
      "settings": {
        "Odoo": {
          "selectedProfile": "my_profile"
        }
      }
    }
  }
```

## Limitations

Due to the lack of features of the Zed API, following options are not available on the Zed plugin:

- Profile selector. You can learn about the "default" profile [here](https://github.com/odoo/odoo-ls/wiki/3.-Configuration-files#no-configuration-file)

  You can change the selected profile in your settings though
- Configuration view and aggregator
- Status widget
- Crash report view
