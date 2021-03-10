import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

Old_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

Old_Profile_ID = WS.getElementPropertyValue(Old_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', FailureHandling.CONTINUE_ON_FAILURE)

println(Old_Profile_ID)

GlobalVariable.RAM_Bundle = '117'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.delay(30)

JAIL_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

JAIL_Profile_ID = WS.getElementPropertyValue(JAIL_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', FailureHandling.CONTINUE_ON_FAILURE)

println(JAIL_Profile_ID)

WS.delay(30)

GlobalVariable.RAM_Recharge = '170000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

WS.delay(30)

GlobalVariable.RAM_Recharge = '500000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

WS.delay(30)

GlobalVariable.RAM_Recharge = '500000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

Migrated_Old_profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

Final_Migrated_Profile_ID = WS.getElementPropertyValue(Migrated_Old_profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', 
    FailureHandling.CONTINUE_ON_FAILURE)

println(Final_Migrated_Profile_ID)

WS.verifyEqual(Old_Profile_ID, Final_Migrated_Profile_ID, FailureHandling.STOP_ON_FAILURE)

WS.comment('Recharge less than and less than or equal to RAM_DEBT flow is working fine.')

