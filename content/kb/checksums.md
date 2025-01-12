---
title: How to check MD* or SHA* checksums with command line
date: 2025-01-12
authorbox: false
sidebar: false
description: How to check MD* or SHA* checksums with command line
categories:
  - Knowledge Base
archives:
  - 2025-01
tags:
  - checksum
  - windows
draft: false
---
Sometimes we need to check MD5 or SHA256 checksums for downloaded files. An easiest way:

```
certutil -hashfile <FILE> MD5
```

or with PowerShell:

```
Get-FileHash <FILE> -Algorithm SHA256
```

<!--more-->
Take a note, the Certutil supports the following hash algorithms for generating checksums:

- MD2
- MD4
- MD5
- SHA1
- SHA256
- SHA384
- SHA512

Get-FileHash in PowerShell supports the following hash algorithms:

- MD5
- SHA1
- SHA256
- SHA384
- SHA512

The default algorithm used by Get-FileHash is SHA256.

---

Certutil (v.10.0.26100.2454) command-line options:

```

Befehle:
  -dump             -- Konfigurationsinformationen oder -datei sichern
  -dumpPFX          -- PFX-Struktur sichern
  -asn              -- ASN.1-Datei analysieren

  -decodehex        -- Decodiert eine hexadezimal-codierte Datei.
  -decode           -- Decodiert eine Base64-codierte Datei.
  -encode           -- Codiert eine Datei mit Base64.

  -deny             -- Verweigert die ausstehende Anforderung.
  -resubmit         -- Übermittelt die ausstehende Anforderung erneut.
  -setattributes    -- Legt Attribute für die ausstehende Anforderung fest.
  -setextension     -- Legt Erweiterung für die ausstehende Anforderung fest.
  -revoke           -- Sperrt das Zertifikat.
  -isvalid          -- Zeigt die aktuelle Zertifikatdisposition an.

  -getconfig        -- Ermittelt die Standardkonfiguration.
  -ping             -- Sendet Signal zur Active
                       Directory-Verwaltungsschnittstelle der Zertifikatdienste.
  -pingadmin        -- Sendet Signal zur Active
                       Directory-Verwaltungsschnittstelle der Zertifikatdienste.
  -CAInfo           -- Zertifizierungsstelleninformationen anzeigen
  -ca.cert          -- Ruft das Zertifizierungsstellenzertifikat ab.
  -ca.chain         -- Ruft die Zertifizierungsstellen-Zertifikatkette ab.
  -GetCRL           -- Ruft die Sperrliste ab.
  -CRL              -- Veröffentlicht neue Sperrlisten [bzw. nur Deltasperrl.].
  -shutdown         -- Fährt die Active Directory-Zertifikatdienste herunter.

  -installCert      -- Installiert das Zertifizierungsstellenzertifikat.
  -renewCert        -- Erneuert das Zertifizierungsstellenzertifikat.

  -schema           -- Bildet das Zertifikatschema ab.
  -view             -- Bildet die Zertifikatansicht ab.
  -db               -- Bildet die binäre Datenbank ab.
  -deleterow        -- Löscht die Serverdatenbankreihe.

  -backup           -- Sichert die Active Directory-Zertifikatdienste.
  -backupDB         -- Sichert die Datenbank des Active
                       Directory-Zertifikatdiensts.
  -backupKey        -- Sichert Active Directory-Dienstezertifikat und privaten
                       Schlüssel.
  -restore          -- Stellt die Active Directory-Zertifikatdienste wieder her.
  -restoreDB        -- Stellt die Datenbank des Active
                       Directory-Zertifikatdiensts wieder her.
  -restoreKey       -- Stellt Active Directory-Dienstezertifikat und
                       priv. Schlüssel wieder her.
  -importPFX        -- Importiert Zertifikat und privaten Schlüssel.
  -dynamicfilelist  -- Zeigt eine dynamische Dateiliste an.
  -databaselocations -- Zeigt den Datenbankpfad an.
  -hashfile         -- Generiert den krypto. Hash einer Datei und zeigt ihn an.

  -store            -- Bildet Zertifikatspeicher ab.
  -enumstore        -- Zertifikatspeicher aufzählen
  -addstore         -- Fügt dem Speicher ein Zertifikat hinzu.
  -delstore         -- Löscht ein Zertifikat aus dem Speicher.
  -verifystore      -- Verifiziert ein Zertifikat im Speicher.
  -repairstore      -- Repariert die Schlüsselzuordnung oder aktualisiert
                       die Zertifikateigenschaften oder
                       Schlüsselsicherheitsbeschreibungen.
  -viewstore        -- Bildet Zertifikatspeicher ab.
  -viewdelstore     -- Löscht ein Zertifikat aus dem Speicher.
  -UI               -- CryptUI aufrufen
  -attest           -- Anforderung des Schlüsselnachweises überprüfen

  -dsPublish        -- Veröffentlicht Zertifikat bzw. Sperrliste in
                       Active Directory.

  -ADTemplate       -- Active Directory-Vorlagen anzeigen
  -Template         -- Registrierungsrichtlinienvorlagen anzeigen
  -TemplateCAs      -- Zeigt Zertifizierungsstellen für Vorlage an.
  -CATemplates      -- Zeigt Vorlagen für Zertifizierungsstellen an.
  -SetCASites       -- Websitenamen für Zertifizierungsstellen verwalten
  -enrollmentServerURL -- Registrierungsserver-URLs für eine
                          Zertifizierungsstelle anzeigen, hinzufügen
                          oder löschen
  -ADCA             -- Active Directory-Zertifizierungsstellen anzeigen
  -CA               -- Registrierungsrichtlinien-Zertifizierungsstellen anzeigen
  -Policy           -- Registrierungsrichtlinie anzeigen
  -PolicyCache      -- Cacheeinträge für Registrierungsrichtlinien anzeigen
                       oder löschen
  -CredStore        -- Anmeldeinformationsspeicher-Einträge anzeigen, hinzufügen
                       oder löschen
  -InstallDefaultTemplates -- Installiert Standardzertifikatvorlagen.
  -URLCache         -- Zeigt URL-Cacheeinsträge an oder löscht diese.
  -pulse            -- Impuls – Autoregistrierungsereignis oder NGC-Aufgabe
  -MachineInfo      -- Zeigt Active Directory-Computerobjektinformationen an.
  -DCInfo           -- Domänencontrolleri-Informationen anzeigen
  -EntInfo          -- Zeigt Unternehmensinformationen an.
  -TCAInfo          -- Zertifizierungsstelleninformationen anzeigen
  -SCInfo           -- Zeigt Smartcard-Informationen an.

  -SCRoots          -- Verwaltet Smartcard-Stammzertifikate.

  -DeleteHelloContainer -- Hello-Anmelde Container löschen. 
     ** Benutzer müssen sich nach Verwendung dieser Option abmelden, damit Sie abgeschlossen werden kann. **
  -verifykeys       -- Verifiziert das öffentlich-private Schlüsselpaar.
  -verify           -- Verifiziert Zertifikat, Zertifikatsperrliste oder Kette.
  -verifyCTL        -- Autorisierungsstamm oder CTL für nicht zulässige
                      Zertifikate überprüfen
  -syncWithWU       -- Mit Windows Update synchronisieren
  -generateSSTFromWU -- SST aus Windows Update generieren
  -generatePinRulesCTL -- CTL für Anheftungsregeln generieren
  -downloadOcsp     -- OCSP-Antworten herunterladen und in Verzeichnis schreiben
  -generateHpkpHeader -- Generiert einen HPKP-Header unter Verwendung von Zertifikaten aus der angegebenen Datei oder dem angegebenen Verzeichnis.
  -flushCache       -- Angegebene Caches im ausgewählten Prozess leeren, z. B. "lsass.exe"
  -addEccCurve      -- ECC-Kurve hinzufügen
  -deleteEccCurve   -- ECC-Kurve löschen
  -displayEccCurve  -- ECC-Kurve anzeigen
  -sign             -- Signiert die Sperrliste bzw. das Zertifikat neu.

  -vroot            -- Erstellt/löscht virtuelle Webstammverz. und Freigaben.
  -vocsproot        -- Erstellt/löscht virtuelle Webstammverz. für
                       OCSP-Webproxy.
  -addEnrollmentServer -- Registrierungsserveranwendung hinzufügen
  -deleteEnrollmentServer -- Registrierungsserveranwendung löschen
  -addPolicyServer  -- Richtlinienserveranwendung hinzufügen
  -deletePolicyServer -- Richtlinienserveranwendung löschen
  -oid              -- Zeigt Namen der Obj.-ID an bzw. legt ihn fest.
  -error            -- Zeigt den Meldungstext des Fehlercodes an.
  -getreg           -- Zeigt den Registrierungswert an.
  -setreg           -- Legt den Registrierungswert fest.
  -delreg           -- Löscht den Registrierungswert.

  -ImportKMS        -- Importiert Benutzers. und Z. zum Archivieren in die DB.
  -ImportCert       -- Importiert eine Zertifikatdatei in eine Datenbank.
  -GetKey           -- Dient zum Abrufen eines Wiederherstellungsblobs des
                       archivierten privaten Schlüssels, zum Generieren eines
                       Wiederherstellungsskripts oder zum Wiederherstellen
                       archivierter Schlüssel.
  -RecoverKey       -- Stellt den archivierten privaten Schlüssel wieder her.
  -MergePFX         -- Führt PFX-Dateien zusammen.
  -ConvertEPF       -- Konvertiert PFX-Dateien in eine EPF-Datei.

  -add-chain        -- (-AddChain) Zertifikatkette hinzufügen
  -add-pre-chain    -- (-AddPrechain) Vorab-Zertifikatkette hinzufügen
  -get-sth          -- (-GetSTH) STH (Signed Tree Head) abrufen
  -get-sth-consistency -- (-GetSTHConsistency) STH (Signed Tree Head)-Änderungen abrufen
  -get-proof-by-hash -- (-GetProofByHash) Nachweis nach Hash abrufen
  -get-entries      -- (-GetEntries) Einträge abrufen
  -get-roots        -- (-GetRoots) Stammelemente abrufen
  -get-entry-and-proof -- (-GetEntryAndProof) Eintrag und Nachweis abrufen
  -VerifyCT         -- Zertifikat-SCT überprüfen
  -?                -- Zeigt diese Syntaxmeldung an.


CertUtil -?              -- Zeigt eine Liste der Befehle an.
CertUtil -dump -?        -- Zeigt den Hilfetext für den Befehl "dump" an.
CertUtil -v -?           -- Zeigt alle Hilfetexte für alle Befehle an.

CertUtil: -?-Befehl wurde erfolgreich ausgeführt.
```
