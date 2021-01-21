<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>j4u-hourly</name>
   <tag></tag>
   <elementGuidId>e8b3948c-d9c2-4aac-b20c-ea0d97f91a78</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \&quot;username\&quot; : \&quot;comj4u\&quot;,\n \&quot;password\&quot; : \&quot;j4u@456\&quot;,\n \&quot;MSISDN\&quot; : \&quot;243814444589\&quot;,\n \&quot;Category\&quot;: \&quot;Hourly\&quot;,\n \&quot;Channel\&quot; : \&quot;WEB\&quot;,\n \&quot;Language\&quot; : \&quot;en\&quot;,\n \&quot;RefNum\&quot; : \&quot;APP1528376639790\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${j4u_Apioffer_Url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_Url</defaultValue>
      <description></description>
      <id>397eb992-cd99-4f36-9bb2-73665a6aa6a7</id>
      <masked>false</masked>
      <name>j4u_Apioffer_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_target_msisdn</defaultValue>
      <description></description>
      <id>e38b620e-f3ec-47c6-8740-af420128f98c</id>
      <masked>false</masked>
      <name>j4u_Apioffer_target_msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_random_msisdn</defaultValue>
      <description></description>
      <id>1fcc6295-f68f-44fc-8ad2-731f4eec2318</id>
      <masked>false</masked>
      <name>j4u_Apioffer_random_msisdn</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
