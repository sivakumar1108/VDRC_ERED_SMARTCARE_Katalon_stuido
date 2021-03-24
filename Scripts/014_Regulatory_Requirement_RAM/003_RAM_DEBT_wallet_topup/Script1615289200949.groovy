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

WS.comment('Amount adjustment should happen like this,paste into calculator and see the results = 1170000-170000+150000+1150000-340000+170000+1170000-170000')

GlobalVariable.RAM_Bundle = '117'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.delay(30)

JAIL_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

JAIL_Profile_ID = WS.getElementPropertyValue(JAIL_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', FailureHandling.CONTINUE_ON_FAILURE)

println(JAIL_Profile_ID)

WS.comment('After this RAM_DEBT wallet should be  = 1170000')

WS.delay(60)

GlobalVariable.RAM_Recharge = '170000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

WS.comment('After this RAM_DEBT wallet should be  = 1000000')

WS.delay(30)

GlobalVariable.RAM_Bundle = '015'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.comment('After this RAM_DEBT wallet should be  = 1150000')

WS.delay(30)

GlobalVariable.RAM_Bundle = '115'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.comment('After this RAM_DEBT wallet should be  = 2300000')

WS.delay(30)

GlobalVariable.RAM_Recharge = '340000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

WS.comment('After this RAM_DEBT wallet should be  = 1960000')

WS.delay(30)

GlobalVariable.RAM_Bundle = '017'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.comment('After this RAM_DEBT wallet should be  = 2130000')

WS.delay(30)

GlobalVariable.RAM_Bundle = '117'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/02_RAM_Bundle purchase'))

WS.comment('After this RAM_DEBT wallet should be  = 3300000')

WS.delay(30)

GlobalVariable.RAM_Recharge = '170000'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/03_RAM_Recharge'))

WS.delay(30)

Final_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

Final_Profile_ID = WS.getElementPropertyValue(Final_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', 
    FailureHandling.CONTINUE_ON_FAILURE)

println(Final_Profile_ID)

WS.comment('Checking the RAM_DEBT top is happening or not like Fianlly RAM_DEBT value should be = 3130000')

WS.comment('Incase Recharge cases are failed = 1170000+150000+1150000+170000+1170000   = 3810000')